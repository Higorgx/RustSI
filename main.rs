
fn main(){
	let  p1 = 1.0;
	let  ma1 = 1.0;
	let  mb1 = 1.0;
	let  p2 = 0.0;
	let  ma2 = 0.0;
	let  mb2 = 0.0;
	let  qda = 0.0; 
	let  qdf = 0.0;

	let  a1 = (p1 * 0.7) + (ma1 * 0.2) + (mb1 * 0.1);
	let  a2 = (p2 * 0.7) + (ma2 * 0.2) + (mb2 * 0.1);
	let mf = (a1 + 2.00 * a2) / 3.00;
	
	let presenca = qda / 100.00 * 75.00;
	if presenca >= qdf {
		if mf < 5.00 && mf >= 3.00{
		println!("Recuperacao");
		}
		if mf <3.00 {
		println!("Reprovado por nota");
		}
		if mf >= 5.00 {
		println!("Aprovado");
		}
	}
	else{
	println!("Reprovado por falta");
	}
	
	println!("The value of A1 is: {}", mf);
	
	
}
