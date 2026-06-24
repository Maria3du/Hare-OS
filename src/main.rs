//para o sistema operacional funcionar de forma decente, é necessário não usar blibiotecas externas enquanto é inicializado.
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
//A ordem estava toda errada, peço desculpas 
mod vgabuffer;

//atualização para o bootloader api
use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();
    loop {}
}

//função de teste 
#[cfg(test)]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    print!("oi :)");
    println!("Running {} tests", tests.len());
    for (i, test) in tests.iter().enumerate() {
        print!("Test {}... ", i);
        test();
        println!("[ok]");
    }
}

//aqui é onde o sistema operacional começa a ser inicializado, nesse momento ele não tem acesso a nada, então é necessário criar uma função que avise caso haja algum problema, no momento essa função ficará em loop.
//como o sistema não tem acesso a blibiotecas externas, é necessário criar uma função que avise caso haja algum problema, no momento essa função ficará em loop, então não seja jumento, pois isso pode trazer danos irreparaveis na sua máquina, um fumo bem grande para você <3
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panic: {}", info);
    loop {}
}

//testes com oatributo no_std
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main(); 
    loop {}
}