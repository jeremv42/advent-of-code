const INPUT_EXAMPLE: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
const INPUT_PART1: &str = "$ cd /
$ ls
113975 bqpslnv
50243 btttmt.nmb
dir gbjh
dir hlpzbht
43500 lblt
dir phpmmtvc
dir plbjmdl
dir tggr
268766 zmllsrzc.qqq
$ cd gbjh
$ ls
64971 dhmprc.qpl
dir jtgbg
dir pzdn
dir slwhsqw
$ cd jtgbg
$ ls
243089 fbfmm
dir hzjcc
dir jgpnm
7952 ljgwgqg
51001 lzwrpqvq.tfq
139239 qbn.gbr
dir smb
dir vvhmmn
15541 zhvgcc
$ cd hzjcc
$ ls
262498 zmllsrzc
$ cd ..
$ cd jgpnm
$ ls
dir php
dir rlp
$ cd php
$ ls
289068 lqdsjjm
$ cd ..
$ cd rlp
$ ls
dir dhlspmh
dir mlsqrz
dir slwhsqw
$ cd dhlspmh
$ ls
249350 gtbp.ttr
$ cd ..
$ cd mlsqrz
$ ls
31876 scwj.cjv
$ cd ..
$ cd slwhsqw
$ ls
2424 vbpwn.qjn
$ cd ..
$ cd ..
$ cd ..
$ cd smb
$ ls
29124 jgpnm.qrq
$ cd ..
$ cd vvhmmn
$ ls
40455 fbfmm
$ cd ..
$ cd ..
$ cd pzdn
$ ls
dir bpdbclp
dir gvvgncqh
dir jfzw
dir nlwnv
$ cd bpdbclp
$ ls
65147 gdrj.qfs
$ cd ..
$ cd gvvgncqh
$ ls
dir fdcdh
dir jnfhsqrl
52531 lblt
dir lprd
dir qzmcfrhq
dir rmbmpjc
$ cd fdcdh
$ ls
285507 vrbhb.fqr
$ cd ..
$ cd jnfhsqrl
$ ls
dir ddzqtsvf
189748 fdcdh.dhj
217915 rpfw.wtt
dir wwrrtc
122077 zctzcb
$ cd ddzqtsvf
$ ls
dir fdcdh
$ cd fdcdh
$ ls
dir fhmpzq
193340 pqq
267704 scwj.cjv
$ cd fhmpzq
$ ls
dir ghzrhzs
198001 thddfc.mlv
$ cd ghzrhzs
$ ls
82916 rjclmm.wcp
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd wwrrtc
$ ls
291755 fdcdh
$ cd ..
$ cd ..
$ cd lprd
$ ls
247564 czqmpt
$ cd ..
$ cd qzmcfrhq
$ ls
102306 bqpslnv
$ cd ..
$ cd rmbmpjc
$ ls
282221 ddvhnf
50544 qcbsqcp.hsp
dir tshl
$ cd tshl
$ ls
592 fdcdh
$ cd ..
$ cd ..
$ cd ..
$ cd jfzw
$ ls
dir fcwsrnjg
dir ftnlwhpn
dir ghqt
52762 qblnjwmq.mgl
45621 zmllsrzc.jcv
$ cd fcwsrnjg
$ ls
81276 mbvcdgvl
$ cd ..
$ cd ftnlwhpn
$ ls
78460 fbfmm
$ cd ..
$ cd ghqt
$ ls
110015 dvtl.nbw
102205 rlblp.zcw
$ cd ..
$ cd ..
$ cd nlwnv
$ ls
dir cvbq
dir jgpnm
226628 scwj.cjv
dir zmllsrzc
$ cd cvbq
$ ls
224362 fvrh.fcp
dir ndjlpwpw
93098 pphz.tmm
30583 qdsgm.hjr
dir qmqlf
$ cd ndjlpwpw
$ ls
17383 ndl.cml
$ cd ..
$ cd qmqlf
$ ls
296208 fdcdh.bwr
246624 lblt
194615 slwhsqw.jhl
dir tsrgs
31676 zmllsrzc.scg
$ cd tsrgs
$ ls
dir fdcdh
164026 jnhfrb.mzm
109383 lblt
138073 zctzcb
$ cd fdcdh
$ ls
128762 lblt
264881 zmllsrzc
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd jgpnm
$ ls
dir bgv
49488 wmp
$ cd bgv
$ ls
dir fpvvw
296599 rdng.ngn
59418 vqvbq.tzz
dir wvdqjn
dir zlwvlpw
$ cd fpvvw
$ ls
215634 gts.znn
12520 lblt
304330 nvd.tlj
84828 qcgqj.mwg
$ cd ..
$ cd wvdqjn
$ ls
142231 zmllsrzc
$ cd ..
$ cd zlwvlpw
$ ls
dir slllz
$ cd slllz
$ ls
201551 scwj.cjv
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd zmllsrzc
$ ls
dir dgdgccc
262704 dwcvsn.lgf
dir fqrcw
277851 rjclmm.wcp
dir zmllsrzc
$ cd dgdgccc
$ ls
276609 bqpslnv.mcr
$ cd ..
$ cd fqrcw
$ ls
dir fghsd
320352 rjclmm.wcp
$ cd fghsd
$ ls
207271 fbfmm
236098 gwvhh.nsv
$ cd ..
$ cd ..
$ cd zmllsrzc
$ ls
58596 rjclmm.wcp
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd slwhsqw
$ ls
206029 bqpslnv.zjg
dir jfp
$ cd jfp
$ ls
51699 zmllsrzc
$ cd ..
$ cd ..
$ cd ..
$ cd hlpzbht
$ ls
302660 bqpslnv
221877 lblt
148517 vpdfdb.vsr
54658 zmllsrzc
$ cd ..
$ cd phpmmtvc
$ ls
dir bqpslnv
dir dptzbgc
311157 fbfmm
162934 gpvnh.tnb
dir ldcqq
dir nld
116676 rjclmm.wcp
dir vnswqbm
dir zmllsrzc
$ cd bqpslnv
$ ls
dir cwbdcvgv
dir dtbbbgw
296450 fbfmm
dir jnfvpnm
dir lmqtmbh
dir lqb
49347 qpt.jsl
dir srq
dir vtnptsl
$ cd cwbdcvgv
$ ls
103874 gqztffsq.vst
116399 sjjstn
$ cd ..
$ cd dtbbbgw
$ ls
dir dptrzvz
17883 fbfmm
dir jgrdzbbh
$ cd dptrzvz
$ ls
dir gfzfhjn
287625 nplcdq.ltn
dir vssdlrp
323499 wlz
$ cd gfzfhjn
$ ls
217616 fbfmm
18148 lblt
79165 rbtvqtrr.dqp
$ cd ..
$ cd vssdlrp
$ ls
86305 lblt
$ cd ..
$ cd ..
$ cd jgrdzbbh
$ ls
dir fdcdh
dir fttv
dir wzwndq
$ cd fdcdh
$ ls
242121 lblt
$ cd ..
$ cd fttv
$ ls
67997 zctzcb
$ cd ..
$ cd wzwndq
$ ls
16310 bqpslnv.rfj
$ cd ..
$ cd ..
$ cd ..
$ cd jnfvpnm
$ ls
17107 lblt
$ cd ..
$ cd lmqtmbh
$ ls
200855 bqpslnv
dir cpdt
304568 rlnf.dfw
$ cd cpdt
$ ls
56206 fdcdh.jrc
138559 jgpnm
123081 rgclnp.vtg
$ cd ..
$ cd ..
$ cd lqb
$ ls
dir sbrzrb
$ cd sbrzrb
$ ls
dir mglsdblq
$ cd mglsdblq
$ ls
172704 rjclmm.wcp
$ cd ..
$ cd ..
$ cd ..
$ cd srq
$ ls
dir bqpslnv
193258 cqslbqml
123266 fbfmm
dir hmhbtnp
dir pcrmfr
27362 pqprb.chw
47189 rjclmm.wcp
288989 slwhsqw
$ cd bqpslnv
$ ls
66777 qqm.jvh
$ cd ..
$ cd hmhbtnp
$ ls
296063 dzm.chg
204474 fbfmm
146902 rjclmm.wcp
$ cd ..
$ cd pcrmfr
$ ls
94907 bqpslnv.wtm
$ cd ..
$ cd ..
$ cd vtnptsl
$ ls
dir bwrbw
12048 djczcg
dir drhf
97998 fdcdh
dir hjljrm
171153 jgpnm.vwr
169093 pzftw.ccl
241263 slwhsqw.ntc
dir thjbhrzj
$ cd bwrbw
$ ls
226255 fdcdh.qzw
283525 pjwv.mql
131501 slwhsqw.gbr
257703 wqfbq
87789 zmllsrzc
$ cd ..
$ cd drhf
$ ls
297259 ffgv.jzr
dir rszprww
12806 zvgmpdnn.psr
$ cd rszprww
$ ls
dir bgsnrdqv
dir grvmtw
251007 scwj.cjv
$ cd bgsnrdqv
$ ls
56538 jdbbfgj.fpw
$ cd ..
$ cd grvmtw
$ ls
68025 trbfdqbz.gdw
$ cd ..
$ cd ..
$ cd ..
$ cd hjljrm
$ ls
dir dzjwf
$ cd dzjwf
$ ls
230855 jgpnm
dir jvd
dir nnwc
dir zmllsrzc
$ cd jvd
$ ls
280910 fztmh
$ cd ..
$ cd nnwc
$ ls
dir bffsm
110991 jgpnm.wbq
dir ttfh
$ cd bffsm
$ ls
dir dwgp
$ cd dwgp
$ ls
5659 bnlvzmbr.tqc
$ cd ..
$ cd ..
$ cd ttfh
$ ls
147196 zpqgfp.qmm
$ cd ..
$ cd ..
$ cd zmllsrzc
$ ls
dir gzmjpctz
$ cd gzmjpctz
$ ls
72856 cffjfsl.mhf
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd thjbhrzj
$ ls
dir fdcdh
260378 lblt
264613 zmllsrzc.pjd
$ cd fdcdh
$ ls
225223 fhfgv.wjn
313245 whs
197514 zctzcb
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd dptzbgc
$ ls
dir nmsdfhfc
dir nmvpwnmh
dir nsqv
148224 plq.mns
101844 scwj.cjv
dir slwhsqw
dir wmm
$ cd nmsdfhfc
$ ls
dir crfw
$ cd crfw
$ ls
90593 vpjd
$ cd ..
$ cd ..
$ cd nmvpwnmh
$ ls
265813 fbfmm
dir fdcdh
267289 fdcdh.gcj
dir hnttb
dir jgpnm
168680 pbfz.zcb
198084 zdlbf
$ cd fdcdh
$ ls
141722 fdcdh.pll
$ cd ..
$ cd hnttb
$ ls
dir gbfvsg
dir jngrjqm
dir slwhsqw
$ cd gbfvsg
$ ls
84991 rjclmm.wcp
55681 slwhsqw.gtl
219041 vtvlz.rws
2947 zmllsrzc.lzr
$ cd ..
$ cd jngrjqm
$ ls
dir bqpslnv
125321 fbfmm
222370 gjpt
$ cd bqpslnv
$ ls
47973 rhzrhrh.mdh
$ cd ..
$ cd ..
$ cd slwhsqw
$ ls
126807 hwp
169701 sjgzj
dir tgpnwrn
286591 wnfjnp.lst
172105 zbrwg.ljs
111461 zmllsrzc.vmj
$ cd tgpnwrn
$ ls
298873 lblt
92666 wjzpj.qzj
$ cd ..
$ cd ..
$ cd ..
$ cd jgpnm
$ ls
135042 fdcdh.fgg
240749 rjclmm.wcp
$ cd ..
$ cd ..
$ cd nsqv
$ ls
233412 pfnvqv.qdn
$ cd ..
$ cd slwhsqw
$ ls
dir nrt
dir spsjgzfr
dir zmllsrzc
$ cd nrt
$ ls
210674 sffpw.gwb
$ cd ..
$ cd spsjgzfr
$ ls
272099 fbfmm
312467 vvtlvcz.qhp
2119 wcmdmqh
$ cd ..
$ cd zmllsrzc
$ ls
242647 fbfmm
307133 lblt
279148 mngdrg.qlq
63394 sgprzhv.vlj
$ cd ..
$ cd ..
$ cd wmm
$ ls
dir bcndl
dir cnlsb
$ cd bcndl
$ ls
dir cqgjzqt
dir hrsdjfv
dir zmllsrzc
$ cd cqgjzqt
$ ls
91704 cfvmd.qnv
$ cd ..
$ cd hrsdjfv
$ ls
240036 bqpslnv.tvl
63562 fbfmm
308727 pnvrr
133855 zctzcb
$ cd ..
$ cd zmllsrzc
$ ls
177997 bqpslnv
$ cd ..
$ cd ..
$ cd cnlsb
$ ls
dir vprqjr
$ cd vprqjr
$ ls
128434 slwhsqw.vbt
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd ldcqq
$ ls
230118 bqpslnv
18831 hdbnpq.mfb
dir nqmwqb
40981 rjclmm.wcp
147598 zctzcb
$ cd nqmwqb
$ ls
313907 dhcphcg.pgp
$ cd ..
$ cd ..
$ cd nld
$ ls
dir bqpslnv
dir chw
317665 fdcdh
321737 slwhsqw.tjb
dir twbt
dir vmvj
79938 wvv.swg
$ cd bqpslnv
$ ls
dir tvsv
$ cd tvsv
$ ls
271095 fbfmm
dir fdcdh
dir jgpnm
208219 ncgzcg.scr
41278 scwj.cjv
10197 tcsjntm.tmr
255687 zctzcb
$ cd fdcdh
$ ls
dir jvcwgbfn
$ cd jvcwgbfn
$ ls
255839 lwnnjz
$ cd ..
$ cd ..
$ cd jgpnm
$ ls
66446 rjclmm.wcp
$ cd ..
$ cd ..
$ cd ..
$ cd chw
$ ls
163229 bqpslnv.cnb
261637 fbfmm
$ cd ..
$ cd twbt
$ ls
180495 sbg.qtm
$ cd ..
$ cd vmvj
$ ls
dir bqpslnv
dir lcjjc
dir wsw
$ cd bqpslnv
$ ls
189635 fbfmm
87919 hgvh.gbq
75372 pht.pjs
198496 rhvqbnc
dir sgjszb
146711 zmllsrzc
$ cd sgjszb
$ ls
dir jlr
177552 tsvzdnwb
$ cd jlr
$ ls
244272 zctzcb
$ cd ..
$ cd ..
$ cd ..
$ cd lcjjc
$ ls
dir cmwm
$ cd cmwm
$ ls
48565 lblt
$ cd ..
$ cd ..
$ cd wsw
$ ls
dir nccwbcj
$ cd nccwbcj
$ ls
203613 djcvsqs.njh
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd vnswqbm
$ ls
dir bmsqrg
dir bqpslnv
17334 dnj.bdf
244204 fbfmm
238971 fdcdh.qjd
dir glb
dir pzrvbvcn
dir qjmtppt
dir rtshvzr
106110 sqsrsph.vwv
dir wtdq
230599 zctzcb
dir zmllsrzc
$ cd bmsqrg
$ ls
289739 mcs
dir vvph
195219 vwrrbpr.mzv
144639 zctzcb
$ cd vvph
$ ls
dir rqpzdtwl
$ cd rqpzdtwl
$ ls
208410 gqzpsbtj
$ cd ..
$ cd ..
$ cd ..
$ cd bqpslnv
$ ls
dir bsrgvwfd
$ cd bsrgvwfd
$ ls
dir tlrgw
236101 zhmsfq
$ cd tlrgw
$ ls
18732 zctzcb
$ cd ..
$ cd ..
$ cd ..
$ cd glb
$ ls
dir fdcdh
227141 fdcdh.ddb
124010 gwvf.thb
31007 lnjwndc.pbf
dir slwhsqw
dir wmp
316521 zctzcb
$ cd fdcdh
$ ls
98547 scwj.cjv
$ cd ..
$ cd slwhsqw
$ ls
dir hcqtzl
$ cd hcqtzl
$ ls
299368 lsvr.ccj
141718 zctzcb
$ cd ..
$ cd ..
$ cd wmp
$ ls
84719 fbfmm
dir hwwlqrh
dir nfbrq
dir slwhsqw
18295 zctzcb
66949 zmllsrzc.spj
$ cd hwwlqrh
$ ls
dir bvfsgfm
dir jbttmc
$ cd bvfsgfm
$ ls
105325 nbnbbf.rbj
$ cd ..
$ cd jbttmc
$ ls
dir tpdnt
$ cd tpdnt
$ ls
256828 cwbwzq
$ cd ..
$ cd ..
$ cd ..
$ cd nfbrq
$ ls
277164 bqpslnv.bzm
$ cd ..
$ cd slwhsqw
$ ls
56736 hwwng.hsr
$ cd ..
$ cd ..
$ cd ..
$ cd pzrvbvcn
$ ls
177454 bvwv.gdg
dir fdcdh
dir mzzgvjs
dir qsdmzl
260924 scwj.cjv
dir sfhpt
190128 slwhsqw
$ cd fdcdh
$ ls
15882 dstdsnr.jrm
62377 lblt
dir wnvgtp
$ cd wnvgtp
$ ls
8890 wrzrp
$ cd ..
$ cd ..
$ cd mzzgvjs
$ ls
324487 dhlgfwcv
141946 dqm.rws
dir mhr
dir pdjn
2675 rjclmm.wcp
dir scdlp
$ cd mhr
$ ls
287618 sdwmpzg.mcq
$ cd ..
$ cd pdjn
$ ls
dir gfhzg
dir mcpzqvgn
$ cd gfhzg
$ ls
dir bqpslnv
dir pqrbn
$ cd bqpslnv
$ ls
204588 rjclmm.wcp
dir ztbmb
$ cd ztbmb
$ ls
180817 zmllsrzc.mbd
$ cd ..
$ cd ..
$ cd pqrbn
$ ls
254533 scwj.cjv
78174 zmllsrzc.hlm
$ cd ..
$ cd ..
$ cd mcpzqvgn
$ ls
117203 hwtps.twz
$ cd ..
$ cd ..
$ cd scdlp
$ ls
dir mgqdbgm
37561 nvb.plr
$ cd mgqdbgm
$ ls
dir fhnz
$ cd fhnz
$ ls
217104 jgpnm.pmw
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd qsdmzl
$ ls
249206 jgpnm
$ cd ..
$ cd sfhpt
$ ls
95502 lblt
148395 qqtvgcdm.wjf
$ cd ..
$ cd ..
$ cd qjmtppt
$ ls
201654 drtgpjh.dzn
36730 hpmp
$ cd ..
$ cd rtshvzr
$ ls
261331 tztfqcl.msd
$ cd ..
$ cd wtdq
$ ls
176817 jwfdct
$ cd ..
$ cd zmllsrzc
$ ls
dir bvqwrs
99444 lblt
72341 qjwdwfdg.vzh
$ cd bvqwrs
$ ls
259109 bjvgfmq.twd
dir rclm
dir zmllsrzc
$ cd rclm
$ ls
157704 cgdtzs.plp
224325 cvh.vms
dir nhflts
9088 slwhsqw
$ cd nhflts
$ ls
dir vmmbsfw
$ cd vmmbsfw
$ ls
22078 slwhsqw
$ cd ..
$ cd ..
$ cd ..
$ cd zmllsrzc
$ ls
199587 cmglvt.cmr
216785 fdcdh
dir gccwrq
$ cd gccwrq
$ ls
68584 ffdd.tsp
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd zmllsrzc
$ ls
213942 bclj.nnv
$ cd ..
$ cd ..
$ cd plbjmdl
$ ls
dir bqpslnv
dir gbs
159730 jgpnm.pdj
dir qdbsj
145186 slwhsqw.hdf
dir tlptvz
226459 vdp
$ cd bqpslnv
$ ls
177530 cbj.lpr
303777 mtbwq.gjs
$ cd ..
$ cd gbs
$ ls
228129 dfdg
$ cd ..
$ cd qdbsj
$ ls
137655 bqpslnv
$ cd ..
$ cd tlptvz
$ ls
dir brssb
303007 ddg
70932 fzqdhn.qsn
dir hhmq
226788 qwvhc.qwj
160118 zctzcb
138524 zmllsrzc.vdm
$ cd brssb
$ ls
222065 fdcdh.lgm
$ cd ..
$ cd hhmq
$ ls
23600 rjclmm.wcp
$ cd ..
$ cd ..
$ cd ..
$ cd tggr
$ ls
223836 lnwhpd.wnl";

struct DirectoryData {
    name: String,
    children: Vec<usize>, // Vec of indices of "all nodes Vec"
}

struct FileMetadata {
    _name: String,
    size: usize,
}

enum Node {
    Directory(DirectoryData),
    File(FileMetadata),
}

fn directory(name: &str) -> DirectoryData {
    DirectoryData {
        name: String::from(name),
        children: Vec::new(),
    }
}

fn node(all_nodes: &mut Vec<Node>, input_line: &str) -> usize {
    let (size, name) = input_line.split_once(" ").unwrap();

    if size == "dir" {
        all_nodes.push(Node::Directory(directory(name)));
    } else {
        all_nodes.push(Node::File(FileMetadata {
            _name: String::from(name),
            size: size.parse::<usize>().unwrap(),
        }));
    }

    all_nodes.len() - 1
}

fn node_directory(all_nodes: &mut Vec<Node>, data: DirectoryData) -> usize {
    all_nodes.push(Node::Directory(data));

    all_nodes.len() - 1
}

fn get_node(all_nodes: &Vec<Node>, id: usize) -> &Node {
    match all_nodes.get(id) {
        Some(node) => node,
        _ => panic!("{} not found", id),
    }
}
fn get_directory(all_nodes: &Vec<Node>, id: usize) -> &DirectoryData {
    match get_node(all_nodes, id) {
        Node::Directory(data) => data,
        _ => panic!("{} is not a directory", id),
    }
}

fn get_node_mut(all_nodes: &mut Vec<Node>, id: usize) -> &mut Node {
    match all_nodes.get_mut(id) {
        Some(node) => node,
        _ => panic!("{} not found", id),
    }
}
fn get_directory_mut(all_nodes: &mut Vec<Node>, id: usize) -> &mut DirectoryData {
    match get_node_mut(all_nodes, id) {
        Node::Directory(data) => data,
        _ => panic!("{} is not a directory", id),
    }
}

fn parse_ls(all_nodes: &mut Vec<Node>, lines: &Vec<&str>, lines_cur: usize, cur_id: usize) -> usize {
    let mut i = lines_cur + 1;
    while i < lines.len() {
        if lines[i].starts_with("$ ") {
            return i - 1;
        }
        let node_id = node(all_nodes, lines[i]);
        get_directory_mut(all_nodes, cur_id).children.push(node_id);
        i = i + 1;
    }
    return i;
}

fn parse_cd(all_nodes: &mut Vec<Node>, line: &str, pwd: &mut Vec<usize>) {
    if line == "$ cd .." {
        pwd.pop();
        return;
    }
    if line == "$ cd /" {
        pwd.clear();
        pwd.push(0);
        return;
    }

    let dest_name = &line[5..];


    let children = get_directory_mut(all_nodes, *pwd.last().unwrap()).children.clone();
    let found_dest = children.iter().find(|id| {
        let n = get_node_mut(all_nodes, **id);
        if let Node::Directory(d) = n {
            d.name == dest_name
        } else {
            false
        }
    });
    let dest: usize = match found_dest {
        Some(dir_id) => dir_id.to_owned(),
        _ => {
            let dir_id = node_directory(all_nodes, directory(dest_name));
            let cur = get_directory_mut(all_nodes, *pwd.last().unwrap());
            cur.children.push(dir_id);
            dir_id
        }
    };
    pwd.push(dest);
}

fn parse_input(input: &str) -> Vec<Node> {
    let mut all_nodes = Vec::<Node>::new();
    node_directory(&mut all_nodes, directory("__root__"));

    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut pwd = Vec::new();
    pwd.push(0); // root

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if line == "$ ls" {
            i = parse_ls(&mut all_nodes, &lines, i, *pwd.last().unwrap());
        } else if line.starts_with("$ cd") {
            parse_cd(&mut all_nodes, line, &mut pwd);
        }
        i += 1;
    }

    all_nodes
}

fn iter_nodes_from(all_nodes: &Vec<Node>, path: &str, parent: &DirectoryData, callback: &mut impl FnMut(&str, &Node, usize)) -> usize {
    let mut total: usize = 0;
    for child_id in &parent.children {
        let mut size: usize = 0;

        let node = get_node(all_nodes, *child_id);
        match node {
            Node::Directory(dir) => size += iter_nodes_from(all_nodes, &format!("{}{}/", path, parent.name), dir, callback),
            Node::File(file) => size += file.size,
        };

        total += size;
        callback(path, node, size);
    }

    total
}
fn iter_nodes(all_nodes: &Vec<Node>, callback: &mut impl FnMut(&str, &Node, usize)) -> usize {
    let root = get_directory(all_nodes, 0);
    let mut total: usize = 0;
    for child_id in &root.children {
        let mut size: usize = 0;

        let node = get_node(all_nodes, *child_id);
        match node {
            Node::Directory(dir) => size += iter_nodes_from(all_nodes, "/", dir, callback),
            Node::File(file) => size += file.size,
        };

        total += size;
        callback("/", node, size);
    }

    total
}

fn part1() {
    let all_nodes = parse_input(INPUT_PART1);
    let mut folders = Vec::<usize>::new();
    let mut callback = |_path: &str, node: &Node, size: usize| {
        match node {
            Node::File(_) => {},
            Node::Directory(_) => folders.push(size),
        }
    };
    iter_nodes(&all_nodes, &mut callback);

    println!("total: {}", folders.iter().filter(|s| **s <= 100000).sum::<usize>());
}

fn part2() {
    const TOTAL_SPACE: usize = 70000000;
    const UPDATE_SPACE: usize = 30000000;

    let all_nodes = parse_input(INPUT_PART1);
    let mut folders = Vec::<usize>::new();
    let mut callback = |_path: &str, node: &Node, size: usize| {
        match node {
            Node::File(_) => {},
            Node::Directory(_) => folders.push(size),
        }
    };
    let free_space = TOTAL_SPACE - iter_nodes(&all_nodes, &mut callback);
    let needed_space = UPDATE_SPACE - free_space;

    println!("best folder size: {}", folders.iter().filter(|s| **s >= needed_space).min().unwrap());
}

fn main() {
    println!("part1");
    part1();
    println!("part2");
    part2();
}
