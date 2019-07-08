extern crate symon;
use symon::PcInfo;
use std::fmt;


struct Logo;

const AL: &'static str =
"
                   -`
                  .o+`
                 `ooo/
                `+oooo:
               `+oooooo:
               -+oooooo+:
             `/:-:++oooo+:
            `/++++/+++++++:
           `/++++++++++++++:
          `/+++ooooooooooooo/` 
         ./ooosssso++osssssso+`
        .oossssso-````/ossssss+`
       -osssssso.      :ssssssso.
      :osssssss/        osssso+++.
     /ossssssss/        +ssssooo/-
   `/ossssso+/:-        -:/+osssso+-
  `+sso+:-`                 `.-/+oso:                            
 `++:.                           `-/+/
 .`                                 `/
";

impl fmt::Display for Logo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", AL )
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1].eq("arch") {
        let al = Logo{};
        println!("{}", al.to_string());
    } else {
        println!("{}", PcInfo::new().to_string());
    }
}