fn main()
{
    /*
    Convert strings to pig latin. The first consonant of each word is moved to the end of the
    word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay”
    added to the end instead (“apple” becomes “apple-hay”).
    */
    
    let some_char_1:Option<char> = Some('C');
    let some_char_2:Option<char> = Some('u');

    println!("{}", is_vowel(some_char_1));
    println!("{}", is_vowel(some_char_2));

    let mut some_str_slice:String = "Divinity".to_string();
    convert_str_pig_latin(&mut some_str_slice);
    let mut some_str:String = String::from("Baldur's Gate");
    println!("{}", some_str);

    str_second_char_becomes_first(&mut some_str);
    println!("{}", some_str);

    let mut neverwinter:String = "Neverwinter".to_string();
    let mut icewind: String = String::from("Icewind");

    convert_str_pig_latin(&mut neverwinter);
    convert_str_pig_latin(&mut icewind);
    println!("{neverwinter}, {icewind}");
}

fn is_vowel(char_to_check:Option<char>) -> bool
{
    let vowels:Vec<char> = vec!['a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'];

    let mut i:usize = 0;
    while i < vowels.len()
    {
        if char_to_check.unwrap() == vowels[i]
        {
            return true;
        };
        i+=1;
    };

    return false;
}

//fn str_second_char_becomes_first(str_to_sort:&str)
fn str_second_char_becomes_first(str_to_modify:&mut String)
{
    let mut current_char_index:usize = 1;
    while current_char_index < str_to_modify.len()
    {
        let current_char:Option<char> = str_to_modify.chars().nth(current_char_index);
        let current_char_to_str:String = current_char.unwrap().to_string();
        str_to_modify.replace_range((current_char_index - 1)..(current_char_index), &current_char_to_str);
        current_char_index+=1;
    }

    str_to_modify.remove(str_to_modify.len() - 1);
}

fn convert_str_pig_latin(str_to_convert:&mut String)
{
    let first_char:Option<char> = str_to_convert.chars().nth(0);

    if is_vowel(first_char)
    {
        let str_to_append:String = String::from("-hay");
        str_to_convert.push_str(&str_to_append);
    } else
    {
        let str_to_append:String = String::from(format!("-{}ay", first_char.unwrap()));
        str_second_char_becomes_first(str_to_convert);
        str_to_convert.push_str(&str_to_append);
    }
}