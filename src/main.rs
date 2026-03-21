//para o sistema operacional funcionar de forma decente, é necessário não usar blibiotecas externas enquanto é inicializado.
#![no_std]
#![no_main]

use core::panic::PanicInfo;
// com o atributo no_mangle a seguir,o nome do compilador não será alterado, sendo possivel chamar a função de "_start".
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    //aqui é onde o sistema operacional começa a ser inicializado, nesse momento ele não tem acesso a nada, então é necessário criar uma função que avise caso haja algum problema, no momento essa função ficará em loop.
    loop {}
}

//como o sistema não tem acesso a blibiotecas externas, é necessário criar uma função que avise caso haja algum problema, no momento essa função ficará em loop, então não seja jumento, pois isso pode trazer danos irreparaveis na sua máquina, um fumo bem grande para você <3
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
