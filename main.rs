fn main() {
    let  p1 = 1.0;
    let  ma1 = 5.0;
    let  ma2 = 5.0;
    let  p2 = 1.0;
    let  mb1 = 5.0;
    let  mb2 = 5.0;
    let  aulas = 30.0;
    let  faltas = 25.0;
    
    
    let media1 = p1*0.7+ma1*0.20+ma2*0.10;
    let media2 = p2*0.7+mb1*0.20+mb2*0.10;
    
    let mediafinal =(media1+(2.0*media2))/3.0;
    
    println!("Nota da P1: {}", p1);
    println!("Nota do Trabalho: {}", ma1);
    println!("Nota do Trabalho: {}", ma2);
    println!("Media: {}", media1);
    println!("");
    
    println!("Nota da P2: {}", p2);
    println!("Nota do Trabalho: {}", mb1);
    println!("Nota do Trabalho: {}", mb2);
    println!("Media: {}", media2);
    println!("");
    println!("Media Final: {}", mediafinal);
    println!("");
    
    println!("Quantidade de aulas: {}", aulas);
    println!("Quantidade de faltas: {}", faltas);
    let frequencia = (faltas/aulas)*100.0;
    println!("Frequencia: {}%", frequencia);
    println!("");
    
    
    if frequencia>=75.0 {
        if mediafinal >= 5.0 {
            print!("Aprovado");
        }else if mediafinal < 5.0 && mediafinal >= 3.0{
                print!("Recuperação");
        } else { 
            print!("Reprovado por nota");
        }
    
    } else {
        print!("Reprovado por frequência");
    }
}
