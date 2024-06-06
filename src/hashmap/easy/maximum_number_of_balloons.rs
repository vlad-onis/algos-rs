/// https://leetcode.com/problems/maximum-number-of-balloons/description/
use std::collections::HashMap;

pub fn max_number_of_balloons(text: String) -> i32 {
    let mut balloon_map: HashMap<char, i32> = HashMap::new();

    let text = text.to_lowercase();

    let keys = vec!['b', 'a', 'l', 'l', 'o', 'o', 'n'];

    // build the balloon map
    for key in keys.clone() {
        balloon_map.insert(key, 0);
    }

    // count the number of balloon letters in the text
    for character in text.chars() {
        if !keys.contains(&character) {
            continue;
        }

        // It is fine to unwrap as we know the key is there for sure
        *balloon_map.get_mut(&character).unwrap() += 1;
    }

    // if any balloon letter was missing in the text return 0
    if balloon_map.values().any(|val| *val == 0) {
        return 0;
    }

    let mut minimum = i32::MAX;

    // Compute the lowest frequency letter among those that are appearing
    // only once in balloon
    for (key, value) in &balloon_map {
        if key != &'l' && key != &'o' {
            if *value < minimum {
                minimum = *value;
            }
        }
    }

    // Find the min frequency of the letters that appear twice
    let minimum_double_letter = balloon_map[&'l'].min(balloon_map[&'o']);

    // compute half of that frequency since we need to use those letters twice
    // to create the balloon word
    let half_double_letter = minimum_double_letter / 2;

    // If other the other letters appears more than 'l' or 'o' we return half
    // of the min frequency of 'l' or 'o' else we return the minimum frequency
    // of the letters that appear only once.
    if minimum > half_double_letter {
        return half_double_letter;
    } else {
        return minimum;
    }
}

#[cfg(test)]
pub mod tests_maximum_number_of_balloons {

    use super::*;

    #[test]
    pub fn test_leetcode1() {
        let text = String::from("krhizmmgmcrecekgyljqkldocicziihtgpqwbticmvuyznragqoyrukzopfmjhjjxemsxmrsxuqmnkrzhgvtgdgtykhcglurvppvcwhrhrjoislonvvglhdciilduvuiebmffaagxerjeewmtcwmhmtwlxtvlbocczlrppmpjbpnifqtlninyzjtmazxdbzwxthpvrfulvrspycqcghuopjirzoeuqhetnbrcdakilzmklxwudxxhwilasbjjhhfgghogqoofsufysmcqeilaivtmfziumjloewbkjvaahsaaggteppqyuoylgpbdwqubaalfwcqrjeycjbbpifjbpigjdnnswocusuprydgrtxuaojeriigwumlovafxnpibjopjfqzrwemoinmptxddgcszmfprdrichjeqcvikynzigleaajcysusqasqadjemgnyvmzmbcfrttrzonwafrnedglhpudovigwvpimttiketopkvqw");
        assert_eq!(max_number_of_balloons(text), 10);
    }

    #[test]
    pub fn test_leetcode2() {
        let text = String::from("mbetypbpefxvviadqaodrbjeoacfomepmzymiudltgnvnpbowwmjgpzzhtiismearuwocsgbiimiqqzaozgeizikrlxmupfzjzmlfttqqbpfblqfkecsdfbsceqjhubfxksivrfwvukapxmuciybfhzlmpeamdxziptxregymqtmgcsujmugissgnlbhxbcxxeoumcqyulvahuianbaaxgzrtmshjguqdaxvxndzoqvwmcjfhpevavnrciqbymnlylbrfkkiceienoarfrzzxtuaqapaeqeqolozadmtgjyhfqzpuaskjuawxqkdqyjqcmbxtvshzrquvegcuyuckznspmrxvqdassidcmrajedsnuuumfwqzvasljlyvfefktiqgvzvdzojtjegsyhbepdkuwvgrfscezvswywmdavpxlekbrlkfnbyvlobazmvgulxrfdranuhomkrlpbfeagfxxxuhjuqhbkhznixquxrxngwimdxdhqbdaouitsvcdmbwxbbaomkgxsqwnexbjjyhtxvkjfqkrrxjghvzqsattubphryqxxdyjkihfnzvjhohnhdlfwoqiwtmwzfgcyhyqtcketvgnbchcxvnhcsoosirfqgdgcsitegzlxdfijzmxnvhrulmgvoqfpzesootscnxenokmmozmoxpaverydbsnimwacjqhrtxkqtvghjyushoctxphxzztukgmnoeycqaeukymvwxcsyvvctflqjhtcvjtxncuvhkptkjnzaetwbzkwnseovewuhpkaxiphdicgacszzdturzgjkzwgkmzzavykancvvzaafgzjhcyicorrblmhsnnkhfkujttbkuuedhwguuaapojmnjdfytdhrepjwcddzsoeutlbbljlikghxefgbqenwamanikmynjcupqpdjnhldaixwygcvsgdkzszmsptqqnroflgozblygtiyaxudwmooiviqcosjfksnevultrf");
        assert_eq!(max_number_of_balloons(text), 14);
    }
}
