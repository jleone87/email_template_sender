//! Easy way to send templated emails to multiple recipients 
//! 
//! Provides the ability to send an email to multiple people with different names while changing the body and header of emails every time.
use std::io::Write;
use std::io;
use std::fmt::{self, Formatter, Display};
use lettre::transport::smtp::authentication::Credentials; 
use lettre::{Message, SmtpTransport, Transport}; 
use lettre::message::MultiPart;


///Struct will represent a person recieving a message with their name and email.
struct Reciever{
    email: String,
    name: String
}
///Display for printing out Reciever classes
impl Display for Reciever{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\tEmail: \t{}Name: {},", self.email, self.name)
    }
}

///Funtion that will ask for at least one reciever. Unlimited recievers can be requested.
fn get_send_list(send_list : &mut Vec<Reciever>){
    let mut email_address = String::new();
    let mut full_name = String::new();
    print!("Please enter the email of your first recipient: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut email_address).unwrap();
    print!("Please enter the name of your first recipient: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut full_name).unwrap();
    send_list.push(Reciever{ email: email_address.clone(), name: full_name.clone() });
    'outer:loop{
        email_address = String::new();
        full_name = String::new();
        print!("Please enter the email or # to stop adding recipients: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut email_address).unwrap();
        if full_name.trim().eq("#") || email_address.trim().eq("#"){
            break 'outer;
        } 
        print!("Please enter the name or # to stop adding recipients: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut full_name).unwrap();
        if full_name.trim().eq("#") || email_address.trim().eq("#"){
            break 'outer;
        } 
        send_list.push(Reciever{ email: email_address.clone(), name: full_name.clone() });
    }
}

fn print_send_list( send_list :& Vec<Reciever>){
    println!("Here is a list of recipients: ");
    for reciever in send_list.iter(){
        println!("{}", *reciever);
    }
}

fn preview_message(username : &String, recipient : &Reciever, header : &String, body : &String){
    println!("Preview message: ");
    println!("To: {}({})", recipient.name, recipient.email);
    println!("From: {}", *username);
    println!("Header: {}", header.replace("<name>", &recipient.name));
    println!("Body:\n{}", body.replace("<name>", &recipient.name));

}

fn main() {
    let mut username = String::new();
    let mut password = String::new();
    let mut header = String::new();
    let mut body = String::new();
    let mut send_list: Vec<Reciever> = Vec::new();

    println!("Hello welcome to the simple template emailing application. This application is meant to send identical emails to multiple people with changed names.");
    print!("Enter your email: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();
    print!("Enter your password: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut password).unwrap();
    let creds = Credentials::new(username.clone(), password.clone()); 
    println!("Simply type <name> wherever you would like a to place the persons name who you are sending it to.");
    print!("Enter the header of your message: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut header).unwrap();
    print!("Enter your message: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut body).unwrap();
    
    get_send_list(&mut send_list);
    print_send_list(&send_list);
    preview_message(&username, send_list.get(0).unwrap(), &header, &body);
    
    println!("Would you like to send your message? type 0 to abort anything else will send");
    let mut decision = String::new();
    io::stdin().read_line(&mut decision).unwrap();
    if decision.trim().eq("0") {
        println!("Have a nice day!");
    }
    else{
        let mailer = SmtpTransport::relay("smtp.gmail.com") 
        .unwrap() 
        .credentials(creds) 
        .build(); 
        for recipient in send_list{
            let email = Message::builder() 
            .from(format!("{}", username.trim()).parse().unwrap()) 
            .to(format!("{} <{}>", recipient.name.trim(),recipient.email.trim()).parse().unwrap()) 
            .subject(header.replace("<name>", recipient.name.trim())) 
            .multipart(MultiPart::alternative_plain_html(
                String::from(""),
                body.replace("<name>", recipient.name.trim()),
            ))
            .unwrap(); 
            match mailer.send(&email) { 
                Ok(_) => println!("Email sent successfully to {}!", recipient.email), 
                Err(e) => panic!("Could not send email: {:?}", e), 
            }
        }
    }
}
