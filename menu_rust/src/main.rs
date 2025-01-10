use std::io::{self, Write};
use chrono::Local;
use dotenv::dotenv;
use std::env;

#[derive(Debug, serde::Serialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, serde::Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

async fn enviar_mensaje_a_chatgpt(mensajes: &Vec<Message>, api_key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    let request = ChatRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: mensajes.clone(),
        temperature: 0.7,
    };

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(response["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("No se pudo obtener una respuesta")
        .to_string())
}

async fn iniciar_chat(api_key: &str) {
    let mut historial_mensajes: Vec<Message> = vec![
        Message {
            role: "system".to_string(),
            content: "Eres un asistente amigable y servicial.".to_string(),
        }
    ];

    println!("\n=== Iniciando chat con ChatGPT ===");
    println!("Escribe 'salir' para volver al menú principal");
    println!("Escribe tu mensaje:");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("Error al leer la entrada");
        let entrada = entrada.trim();

        if entrada.to_lowercase() == "salir" {
            println!("Volviendo al menú principal...");
            break;
        }

        // Agregar mensaje del usuario al historial
        historial_mensajes.push(Message {
            role: "user".to_string(),
            content: entrada.to_string(),
        });

        // Obtener respuesta de ChatGPT
        match enviar_mensaje_a_chatgpt(&historial_mensajes, api_key).await {
            Ok(respuesta) => {
                println!("ChatGPT: {}", respuesta);
                
                // Agregar respuesta al historial
                historial_mensajes.push(Message {
                    role: "assistant".to_string(),
                    content: respuesta,
                });
            },
            Err(e) => {
                println!("Error al comunicarse con ChatGPT: {}", e);
            }
        }
        println!(); // Línea en blanco para mejor legibilidad
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("No se encontró la API KEY de OpenAI");

    loop {
        println!("\nPor favor, selecciona una opción:");
        println!("1. Saludar");
        println!("2. Despedirse");
        println!("3. Mostrar fecha y hora actual");
        println!("4. Chatear con ChatGPT");
        println!("5. Salir");

        print!("\nIngresa tu opción (1-5): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error al leer la entrada");

        let opcion: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("¡Por favor ingresa un número válido!");
                continue;
            }
        };

        match opcion {
            1 => println!("¡Hola! ¿Cómo estás hoy?"),
            2 => println!("¡Hasta luego! Que tengas un buen día"),
            3 => {
                let fecha_actual = Local::now();
                println!("Fecha y hora actual: {}", fecha_actual.format("%d/%m/%Y %H:%M:%S"));
            },
            4 => {
                iniciar_chat(&api_key).await;
            },
            5 => {
                println!("¡Gracias por usar el programa! ¡Hasta pronto!");
                break;
            }
            _ => println!("Opción no válida. Por favor selecciona un número del 1 al 5"),
        }
    }
}