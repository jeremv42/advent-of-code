const INPUT_EXAMPLE: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
const INPUT_PART1: &str = "dcbcsbblhhgdgssmcmqccdwdvwdvvcfvcfvvpvmmwccdqddshhdppcfpfbbfggfjfvvhzhmmsqswsttcgtctggjllhnnqbnnnldlglplvlbvbzztpzzvpphshwwfhhgssvgsvvpfvfzznbznndqqtsqttnvtvqqqrgqrrgzzlqzzbvzbzmzffnbfnbffcggdcggqhqpqzpqzqbbqvbqbhhtzhhrrrprlrclrcrssqspqpbblggfvvbpvbbssvttsztzpzjjwffnpnfnrrqhqgqrrzqzbzzdpdjpplpclpptltztjtztqthtmmmpsmmpjpqpvvshhsfshhrvrdvdggwssdbbzbttlmlmpprmrmlrrmqrqsshjhjljzjnjllbbvjvnnppqbppqqfffrbfbdfbbqppmtppfttqddtzzwbblqqbddgwgqwqzwwpzwzjjbccdjdwwrggrmrtmrtmrrvpptwwcdcwwsdshhjjhwjwqjwwqsshrrbsblssphspsjpssnsmsqswqwfwjwccdvccrqqnsqnsnwsnsmnmccvcqqfjjfljfljfjgjffjvffvjvhjhfhrfrlrrtpphbhwbbvmmnpndnlljdjfjsffvccnhchwhddrccljllmnnhmnmpnnhtnthnhvnhnwnqqjmmgzzvssphhjljbjwjbwwdnwwdvvzpvvhccvjjlclrrppgjgsshghvvzjvvpvvbjjqhjqqfcqffchcmmvdvtddftdtppfnppcnpnznwwpdpjpnpttlbbmmqfqmffqnfnbffbrbgbwggsmsbbfgbbfbzbtzbbvmmdlmlrlnlwwbswsjjpjhhwqwzqznqzqvvwccwvwwcbbpvvwcvcvzzhzchzccbnccjlljsjbjdbbvqbvvfdfgdfggzfzlzmzrznnqsnncmnnljjtcjcfftrthtrhrgrvvjggzpggrcrpcppggdfggpmmwnmnlmlglvvmjvvddsmsjsnjssqffffctczcjzjjlbbhpbbfdbdzbbsjbjnjbbqqjbqjbjjhwhssmvsschhhwppbjppllprpjrppdpfpgpjptjtptplpnptnptpjjmcmnnnhnjhjlhjlhjlhlwwhwmwjjbhhlfffrbbgvvqwvwswtstmmmfpmfpffmcfcrrqsqzqjjjnpnnztnznddlwdldsdbssstzstswwjtttmmwmsmwssvswszsjzsjjsffmccmfccqzzvpmbbbsqffgzdqbjtzhlqdzhlpwghlstcrcrffrnbwjnqgmbmpgttfmsswdqctlrpdnlsgnlldvbfpwtcptvbwftzcnbbscrrcpnwtmllcvsrmwzzlsdmfctdcwsqdlsnzgfpmzrnswhbqjhstztzzmzpcttsgsggnlhvjmcbbrhgqhsfmglpcbdvmmmnfbtbfrqbpcmttjtnwvznbshwrmznnsvpjqjntlzspljnbwtjcqztsfcqlrggrpzjgjsvqqcrmrjmzwdsshqfhbtfmlwmfvtbcgdmjgtcnphfmfmjlbjzrvjslccftnwcchgdwjnlthlwgldjwqwgdptdjdmzdrrzcdpbfrtdgcspjtqdqvzswwdwrhggdrqjjgwwrbwhhlrpqmszvlvjfqptncjlscvzbgzgmsttlbhbfrctnsphjcrcwlhcgrcrsjbrjvptgfbjgjrvtzmnhpzcgptbmrgvstsltnctjphsjdwpdqblfswzfhgjbpfrptlmhfwcpdlzqccgtdvbzhwngrhlqftmlhjprscflgzpflvvpfsjmlnmbzsrlrshvnsqrhhlqdlzhrcbjjjfrbqcdspwsmltcrtlbdnbnvhbbwgqdcncsztbfwztzdbqgcrnvndmpstpncbwvtctzdpmcpvrgqvjjztfwpvjtdqlvcvdpfzgcghsmbcwtzztmqwdpsprgsmfhphqsqmflrjdqzjscgzlnvcwcrlmpdscnhqpqjfdbdftqgttwntdbpnshdwnmwsrslfgnzlnwwwqgdbfnthhqtvbzsqgzjhhghtmvvfhlmlpbghnsvlttzsjlgndhdqmqqfdlqnbfscsnnqzcdwzdlqcnstcbsffghftqvwrsshgwmlnprhdnnwslwfwtmtfzdjwpmlvvdhjvdwrhvdsmpgrdpnqsjpqmhttrmdwllrmnbznwjwvvpjnnbnfzbdnhjbqqrnzgdqbspbqtwdpgsbwpzfdbpvzfjpsgmztnzrpvvhwlfscfvfpfblvplgdbhvjjpdjtnwrmvpjsphvglpsntvwtqwqvprcgwjltddpjngvmzfzhmqnnwglbzsbrcztpplpsmgmcfgzgpbtgsjrfvdzzcsthznvdpbwvdlcgdhncsjdvpcbbmtrqczctjljdfghsrvrsfjglqqhjttfdhqdqwhzhqrggsjwlldrntwmmbftgqjhpvvctpbtgltnttlhdqbsbwcqmctwlsnhmhncpmnzsllmjhcgnlfgvpcqrwzvsstgwbvjtnrlbblclzdrcwddrwnptqzgwdwtgrfwffpzjmwbqfmmrcfzfzjbwsslbhggtwtcrlhffzgfgrtljgnznnlgzwfmtwwqzhlthvpfclrtmrqfzrggbctzfsmjhrtwzpfrnhwwprnbwmvwvmvnzqpggmzgslctqbqdtvhgzjwzsnblqzmcpnpwllzhvzzmfqzfjlrsrcnjzqdzbpjftljtzvvmrjszvqllnhhgnrnqttnwlvllphjtnmlwqhcvmbsvnwtcdmhsmhdcwqwtvggcqfpdsrsscmhcvfzvdnffrfhdfmgbsghdbpwdwrdmlvsnzwcfchcqvccszdqrbnfvrpbftcwnjmczwgqzmtlmrthlhtjpmchltcmcqwgfgshtvtpmcmpbmlnjmhnpdsljjmjzddnlgtnjqztzsqqlhtqcscjvjncjvfcvsgjhqgzrmtjjcgvvmwswffmlcvlhqhbvrldrvbrfmmqcnqmfzlsghvclrdbsvcbqspgbzmjnlrdhvncnbcfmdlqrssggsmlwwglcjjzmcdwcnrgvvmgjcfbzlncmgqtllgldbdztbtfcjczqtjjlnpqmrgtjsmrbscvqlgqghfbgwccgwrjrzrsbbrqnjcqhllsqmrjtzmjnndwwmdjfhspjpgdcjjpdvwrsgjcfnpllnnnccvnfvqbpddgvbgbsbczmbzrbclczljdmbhpgmhwlqvnjzjwzzfjmsmcchjrqrtmhwlgjsptctlbtdnrqgntcvngcrqdqptfmhbvlhrqchdtwdwbrbqwtswzcrrfndldwmjbczppzrnncvvqsmpvvqcnsvhprhlmrhnjbwdvrbbwwdtmzrqttschrztgjcshlhfbmhmwrrmwgmfshpdhjwgdmcfdvqrmmwgmzbrlgbfltvlzmvqbgvhppdzglqbdlrhjnntnfvtmzjqccmqdbpjqphfggmjqdrndhclcfvqsjrsbcgdhcrbjsdjmwrzvpfpcjwjfdltztfzgmlzqzmztpvbflppgnhdzssznngfjggczdtdmcczjzfsnflpwqrbggmqdbbprfptzcdqvhrvszzjqqjlrlrdpwcnzhvgfhpcdbbgsfmtnszwbhwcdwdghqqctwqlqrlqbdwfvjnhcpmzchqfrwzhzgslzhncmrlrpzcjczvzwcvwcldbmscfqnnqnwwvrpfvjswqmhgmhgnmfzpsjmdhbpvsftccttvdpcdzcnzswqmtrwbctpbgmzrvrrshjjgdqsqrwfpcmsbvqhccvjqpztlttwjjdtbmwslschpqjjllvjcjwtmrtvvwdzglstvtpndmmzcpgqsvqgfdtqdjdctsbsbmqzqhtczhgqgwbdlrhjrwcmtbzfndsbnpnhmsdhtghwwzvtdwtscdnwzmrjrsrjvvbvrpbszchwbltrjbcqmlhqnzcfbhjqnsjghlnlbsrgzrrzfvslwpbqmgfswhgjdsdfrzsdhvdqvqfbcbvmbfhjfpzwlbspcbnvrgpfnmjbwbsnpqpqhjpsnwrlcmfhpdmjbvpnctcfqgdmwzblsnr";

fn is_diff4(slice: &str) -> bool {
    if slice.len() != 4 {
        return false;
    }

    !slice[1..].contains(&slice[0..1]) && !slice[2..].contains(&slice[1..2]) && !slice[3..].contains(&slice[2..3])
}

fn get_packet_index(data: &str) -> usize {
    for i in 4..data.len() {
        if is_diff4(&data[(i - 4)..i]) {
            return i
        }
    }
    return 0
}

fn part1() {
    let pos = get_packet_index(INPUT_PART1);
    println!("packet start index: {}", pos);
}

fn is_diff14(slice: &str) -> bool {
    if slice.len() != 14 {
        return false;
    }

    for i in 1..14 {
        if slice[i..].contains(&slice[(i - 1)..i]) {
            return false;
        }
    }
    true
}

fn get_message_index(data: &str) -> usize {
    for i in 14..data.len() {
        if is_diff14(&data[(i - 14)..i]) {
            return i
        }
    }
    return 0
}

fn part2() {
    let pos = get_message_index(INPUT_PART1);
    println!("packet message index: {}", pos);
}

fn main() {
    println!("part1");
    part1();
    println!("part2");
    part2();
}
