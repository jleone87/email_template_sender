//! Easy way to send templated emails to multiple recipients on gmail
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
        write!(f, "\tEmail: {}, Name: {}", self.email.trim(), self.name.trim())
    }
}

///Funtion that will ask for at least one reciever. Unlimited recievers can be requested.
fn get_send_list(send_list : &mut Vec<Reciever>){
    //creates two variables that will temporarily house emails and names
    let mut email_address = String::new();
    let mut full_name = String::new();
    //prompt for first email
    print!("Please enter the email of your first recipient: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut email_address).unwrap();
    //prompt for first name
    print!("Please enter the name of your first recipient: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut full_name).unwrap();
    //fills our send_list vec with a new reciever entry
    send_list.push(Reciever{ email: email_address.clone(), name: full_name.clone() });
    //creates a named loop to repeatedly ask for names
    'outer:loop{
        //clears our email and name
        email_address = String::new();
        full_name = String::new();
        //prompt for email
        print!("Please enter the email or # to stop adding recipients: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut email_address).unwrap();
        //checks if quit character was inputted
        if email_address.trim().eq("#"){
            break 'outer;
        } 
        //prompt for name
        print!("Please enter the name or # to stop adding recipients: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut full_name).unwrap();
        //checks if quit character was inputted
        if full_name.trim().eq("#"){
            break 'outer;
        } 
        send_list.push(Reciever{ email: email_address.clone(), name: full_name.clone() });
    }
}
///Prints a list of recipient.
fn print_send_list( send_list :& Vec<Reciever>){
    println!("Here is a list of recipients: ");
    //iterates through the send_list and prints each entry
    for reciever in send_list.iter(){
        println!("{}", *reciever);
    }
}
///prints a plain text preview of the FIRST email being sent
fn preview_message(username : &String, recipient : &Reciever, header : &String, body : &String){
    println!("Preview message: ");
    println!("To: {}({})", recipient.name, recipient.email);
    println!("From: {}", *username);
    //.replace changes <name> to the actual recipients name
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
    //user email prompt
    print!("Enter your email: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();
    //user password prompt
    print!("Enter your password: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut password).unwrap();
    //creates a cred object
    let creds = Credentials::new(username.clone(), password.clone()); 
    //creates connection with gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com") 
        .unwrap() 
        .credentials(creds) 
        .build(); 
    println!("Simply type <name> wherever you would like a to place the persons name who you are sending it to.");
    //prompt for header
    print!("Enter the header of your message: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut header).unwrap();
    //prompt for body
    print!("Enter your message: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut body).unwrap();

    //calls get_send_list to fill the send_list vector with recipients
    get_send_list(&mut send_list);
    //prints out the recipients and preview message
    print_send_list(&send_list);
    preview_message(&username, send_list.get(0).unwrap(), &header, &body);

    //confirmation prompt
    println!("Would you like to send your message? type 0 to abort anything else will send");
    let mut decision = String::new();
    io::stdin().read_line(&mut decision).unwrap();
    //checks if the quit character was used
    if decision.trim().eq("0") {
        println!("Have a nice day!");
    }
    //sends email
    else{
        //loops through our send_list vector
        for recipient in send_list{
            //initializes a Message object
            let email = Message::builder() 
            .from(format!("{}", username.trim()).parse().unwrap()) 
            .to(format!("{} <{}>", recipient.name.trim(),recipient.email.trim()).parse().unwrap()) 
            //.replace changes <name> to actual recipient name
            .subject(header.replace("<name>", recipient.name.trim())) 
            //this is our body, you can only send html emails with multiparts
            .multipart(MultiPart::alternative_plain_html(
                //creates an empty string to satisfy the multiparts first argument
                String::from(""),
                body.replace("<name>", recipient.name.trim()),
            ))
            .unwrap(); 
            //email attempts to send
            match mailer.send(&email) { 
                Ok(_) => println!("Email sent successfully to {}!", recipient.email.trim()), 
                Err(e) => panic!("Could not send email: {:?}", e), 
            }
        }
    }
}
