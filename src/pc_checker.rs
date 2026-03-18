use hardware_query::{HardwarePresets, HardwareInfo};

use std::process::Command;
use std::path::PathBuf;

pub struct UserSystem {
    // RAM
    pub ram_gb: u64,
    // CPU
    pub cpu_name: String,
    pub cpu_cores: usize,
    pub cpu_threads: usize,
    pub has_avx2: bool,
    pub has_fma: bool,
    pub has_amx: bool,
    pub cpu_ai_capable: bool,
    // GPU
    pub gpu_name: Option<String>,
    pub gpu_vram_gb: Option<u64>,
    pub has_cud: bool,
    pub has_rocm: bool,
    pub has_directml: bool,
    pub has_metal: bool,
    pub gpu_ai_capable: bool,
    // DISK
    pub disk_free_gb: Option<u64>,
    //SCORE
    pub ai_score: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ai_assessment = HardwarePresets::ai_assessment()?;
    let hw_info = HardwareInfo::query()?;

    let cpu = ai_assessment.overview.cpu;

let path: PathBuf = std::env::current_dir()?;
println!("Текущая директория: {:?}", path);
let output = Command::new("cmd")
    .args(["/C", "wmic logicaldisk get size,freespace,caption"])
    .output()?;

println!("{}", String::from_utf8_lossy(&output.stdout));
println!("Текущая директория: {:?}", path);


    println!("ПРОВЕРКА RAM:");
    println!("RAM: {} GB", ai_assessment.overview.memory_gb);
    println!("");

    println!("ПРОВЕРКА ЦПУ:");
    println!("Модель: {}", cpu.name);
    println!("Количество ядер: {}", cpu.cores);
    println!("Количество потоков: {}", cpu.threads);
    
    let cpu_info = hw_info.cpu();
    let ai_instructions = [
        ("AVX2"),
        ("FMA"),
        ("AMX"),
    ];
    println!("");

    println!("Глянем NPU (Neural Processing Unit, нейронный процессор):");
    for instr in ai_instructions {
        if cpu_info.has_feature(instr) {
            if instr == "AVX2"{
                println!("  {} - Отлично, это минимальный стандарт для комфортного запуска LLM. Ускоряет матричные умножения в нейросетях.", instr);
            }
            else if instr == "FMA"{
                println!("  {} - Замечательная штука для ускорения вычисления матриц, на них как раз LLM и строится", instr);
            }
            else if instr == "AMX"{
                println!("  {} - Откуда у тебя это вообще O_o", instr);
            }
        }
    }
    
    if cpu.ai_capable {
        println!("Процессор подходит для ИИ ヾ(≧ ▽ ≦)ゝ");
    } else {
        println!("Процессор НЕ подходит для ИИ ~(>_<。)＼");
    };
    println!("");

    println!("ПРОВЕРКА ГПУ:");
    match ai_assessment.overview.gpu {
        Some(gpu) => {
            println!("  Модель: {}", gpu.name);
            println!("  VRAM: {} GB", gpu.vram_gb);
            

for gpu_info in hw_info.gpus() {
    if gpu_info.model_name().contains(&gpu.name) {
        println!("Поддержка AI:");
        
        
        if gpu_info.supports_cuda() {
            println!("есть CUDA");
        } else {
            println!("нет  CUDA");
        }
        
        
        if gpu_info.supports_rocm() {
            println!("есть ROCm");
        } else {
            println!("нет  ROCm");
        }
        
        
        if gpu_info.supports_directml() {
            println!("есть DirectML");
        }
        
        
        if gpu_info.supports_metal() {
            println!("есть Metal");
        }
    }
}
            
            if gpu.ai_capable {
                println!("GPU подходит для ИИ ヾ(≧ ▽ ≦)ゝ");
            } else {
                println!("GPU НЕ подходит для ИИ ~(>_<。)＼");
            };
        }
        None => {
            println!("GPU не найдено (◔_◔)");
        }
    }
    println!("");

    if ai_assessment.overview.environment != "Native" {
        println!("Запустите в нативной среде, а не на эмуляторе (⓿_⓿)");
        println!("Текущая среда: {}", ai_assessment.overview.environment);
    }

    println!("Оценка производительности системы: {}%", 
             ai_assessment.overview.performance_score);
    println!("Оценка для работы ИИ: {}%", ai_assessment.ai_score);

    Ok(())
}