use hardware_query::{HardwarePresets, HardwareInfo};
use sysinfo::{Disks};

pub struct DiskInfo {
    pub mount_point: String,
    pub total_gb: u64,
    pub free_gb: u64,
}
pub struct UserSystem {
    // RAM
    pub ram_gb: f64,
    // CPU
    pub cpu_name: String,
    pub cpu_vendor: String,
    pub cpu_cores: u32,
    pub cpu_threads: u32,
    pub has_avx2: bool,
    pub has_fma: bool,
    pub has_amx: bool,
    pub cpu_ai_capable: bool,
    // GPU
    pub gpu_name: Option<String>,
    pub gpu_vendor: Option<String>,
    pub gpu_vram_gb: Option<f64>,
    pub has_cuda: bool,
    pub has_rocm: bool,
    pub has_directml: bool,
    pub has_metal: bool,
    pub gpu_ai_capable: bool,
    // DISK
    pub disks: Vec<DiskInfo>,
    //SCORE
    pub ai_score: u8,

    pub environment: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let system_info = system_checker()?;
    get_info(system_info);
    Ok(())
}

fn system_checker() -> Result<UserSystem, Box<dyn std::error::Error>> {
    let ai_assessment = HardwarePresets::ai_assessment()?;
    let hw_info = HardwareInfo::query()?;
    let cpu = ai_assessment.overview.cpu;
    let cpu_info = hw_info.cpu();

    //RAM
    let ram_gb = ai_assessment.overview.memory_gb;

    //CPU
    let cpu_name = cpu.name;
    let cpu_vendor = cpu.vendor;
    let cpu_cores = cpu.cores;
    let cpu_threads = cpu.threads;
    let has_avx2 = cpu_info.has_feature("AVX2");
    let has_fma = cpu_info.has_feature("FMA");
    let has_amx = cpu_info.has_feature("AMX");
    let cpu_ai_capable = cpu.ai_capable;

    //GPU
    let (gpu_name, gpu_vendor, gpu_vram_gb, has_cuda, has_rocm, has_directml, has_metal, gpu_ai_capable) =
        match ai_assessment.overview.gpu {
            Some(gpu) => {
                let mut cuda = false;
                let mut rocm = false;
                let mut directml = false;
                let mut metal = false;
                
                for gpu_info in hw_info.gpus() {
                    if gpu_info.model_name().contains(&gpu.name) {
                        // Определяем производителя
                        let vendor = gpu.vendor.as_str();
                        
                        match vendor {
                            "NVIDIA" => {
                                cuda = gpu_info.supports_cuda();
                                directml = gpu_info.supports_directml();
                            }
                            "AMD" => {
                                rocm = gpu_info.supports_rocm();
                                directml = gpu_info.supports_directml();
                            }
                            "Intel" => {
                                directml = gpu_info.supports_directml();
                            }
                            "Apple" => {
                                metal = gpu_info.supports_metal();
                            }
                            _ => {
                                cuda = gpu_info.supports_cuda();
                                rocm = gpu_info.supports_rocm();
                                directml = gpu_info.supports_directml();
                                metal = gpu_info.supports_metal();
                            }
                        }
                        break;
                    }
                }
                
                (
                    Some(gpu.name),
                    Some(gpu.vendor),
                    Some(gpu.vram_gb),
                    cuda,
                    rocm,
                    directml,
                    metal,
                    gpu.ai_capable
                )
            }
            None => (None, None, None, false, false, false, false, false)
        };

    //DISK
    let disks = get_all_disks_info();

    //SOME
    let ai_score = ai_assessment.ai_score;
    let  environment = ai_assessment.overview.environment;
    Ok(UserSystem {
        ram_gb,
        cpu_name,
        cpu_vendor,
        cpu_cores,
        cpu_threads,
        has_avx2,
        has_fma,
        has_amx,
        cpu_ai_capable,
        gpu_name,
        gpu_vendor,
        gpu_vram_gb,
        has_cuda,
        has_rocm,
        has_directml,
        has_metal,
        gpu_ai_capable,
        disks,
        ai_score,
        environment,
    })    
        }

fn get_all_disks_info() -> Vec<DiskInfo> {
    let mut disks_info = Vec::new();
    let disks = Disks::new_with_refreshed_list();
    
    for disk in disks.list() {
        let mount_point = disk.mount_point().to_string_lossy().to_string();
        let total_gb = disk.total_space() / 1024 / 1024 / 1024;
        let free_gb = disk.available_space() / 1024 / 1024 / 1024;
        

        if total_gb > 1 {
            disks_info.push(DiskInfo {
                mount_point,
                total_gb,
                free_gb,
            });
        }
    }
    
    disks_info
}

fn get_info(sys:UserSystem){
    println!("{}", "=".repeat(50));
    println!("Loliimilk Software 2026 x Shikaru AI");
    println!("MIT License\n
    Copyright (c) 2026 Валентин Вольфович");
    println!("{}", "=".repeat(50));

    // Окружение
    println!("\nОКРУЖЕНИЕ:");
    if sys.environment != "Native"{
        println!("Запустите в нативной среде, а не на эмуляторе (⓿_⓿)");
        println!("Текущая среда: {}", sys.environment);
    } else {println!("  Тип: {}", sys.environment);}
    
    // Память
    println!("\nОПЕРАТИВНАЯ ПАМЯТЬ:");
    println!("  RAM: {:.1} GB", sys.ram_gb);
    
    // Процессор
    println!("\nПРОЦЕССОР:");
    println!("  Модель: {}", sys.cpu_name);
    println!("  Производитель: {}", sys.cpu_vendor);
    println!("  Ядра: {}", sys.cpu_cores);
    println!("  Потоки: {}", sys.cpu_threads);
    
    // CPU расширения
    println!("  Поддержка инструкций:");
    println!("    AVX2: {}", if sys.has_avx2 { "- Поддерживается, отлично, это минимальный стандарт для комфортного запуска LLM. Ускоряет матричные умножения в нейросетях." } 
            else { " - Не поддерживается, жаль, это минимальный стандарт для комфортного запуска LLM. Ускоряет матричные умножения в нейросетях." });
    println!("    FMA: {}", if sys.has_fma { "- Поддерживается, замечательная штука для ускорения вычисления матриц, на них как раз LLM и строится" } 
            else { " - Не поддерживается, жаль, это замечательная штука для ускорения вычисления матриц, на них как раз LLM и строится" });
    if sys.has_amx{
        println!("    AMX: - Откуда у тебя это вообще O_o");
    }
    println!("  AI Capable: {}", if sys.cpu_ai_capable { "CPU подходит для ИИ ヾ(≧ ▽ ≦)ゝ" } else { "CPU НЕ подходит для ИИ ~(>_<。)＼" });
    
    // Видеокарта
    println!("\nВИДЕОКАРТА:");
    match sys.gpu_name {
        Some(name) => {
            println!("  Модель: {}", name);
            println!("  Производитель: {}", sys.gpu_vendor.unwrap_or_else(|| "Неизвестно".to_string()));
            println!("  VRAM: {:.1} GB", sys.gpu_vram_gb.unwrap_or(0.0));
            println!("  Поддержка API:");
            println!("    CUDA: {}", if sys.has_cuda { "Да" } else { "Нет" });
            println!("    ROCm: {}", if sys.has_rocm { "Да" } else { "Нет" });
            println!("    DirectML: {}", if sys.has_directml { "Да" } else { "Нет" });
            println!("    Metal: {}", if sys.has_metal { "Да" } else { "Нет" });
            println!("  AI Capable: {}", if sys.gpu_ai_capable { "GPU подходит для ИИ ヾ(≧ ▽ ≦)ゝ" } else { "GPU НЕ подходит для ИИ ~(>_<。)＼" });
        }
        None => {
            println!("  GPU не обнаружнена (◔_◔) или не поддерживается");
        }
    }
    
    // Диск
    println!("\nДИСКОВОЕ ПРОСТРАНСТВО:");
    if sys.disks.is_empty() {
        println!("   Нет информации о дисках");
    } else {
        println!("   {:<14} {:>8} {:>14}", "Диск", "Всего", "Свободно");
        println!("   {}", "-".repeat(45));
        
        for disk in &sys.disks {
            
            println!("   {:<14} {:>6} GB {:>8} GB", 
                disk.mount_point,
                disk.total_gb,
                disk.free_gb,
            );
    }};

    println!("\nОЦЕНКА ДЛЯ AI:\n");
    
    let score = sys.ai_score;
    let bar_length = 30;
    let filled = (score as f64 / 100.0 * bar_length as f64) as usize;
    let bar: String = (0..bar_length).map(|i| if i < filled {"+"} else {"-"}).collect();
    println!("  [{}{}] {}/100", bar, if score == 100 {""} else {""}, score);

    if sys.ai_score >= 80 {
        println!("  Отлично подходит для AI задач (оценка: {}/100\n)", sys.ai_score);
    } else if sys.ai_score >= 50 {
        println!("  Средняя производительность для AI (оценка: {}/100\n)", sys.ai_score);
    } else {
        println!("  Слабая производительность для AI (оценка: {}/100)\n", sys.ai_score);
    }
}