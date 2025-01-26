use figlet_rs::FIGfont;

pub fn ascii_print (){

    let standar_font = FIGfont::standard().unwrap();

    let figure = standar_font.convert("QR Console APP");

    if let Some (art)= figure {
        println!("{}", art);
    }else {
        println!("Could't generate art ascii");
    }
}