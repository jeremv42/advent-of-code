use iterchunks::IterArrayChunks;

const INPUT_EXAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

const INPUT_PART1: &str = "zBBtHnnHtwwHplmlRlzPLCpp
vvhJccJFGFcNsdNNJbhJsJQplQMRLQMlfdfTPCLfQQCT
GPhjcjhZDjWtnSVH
BNhHVhrGNVTbDHdDJdJRPJdSQQSJwPjR
lvtsfbsqzwSnJcvjSm
MftttFLftZMLgtgMbltMqZzbDNrTpVGhNWrDTrpTGNpZGZhD
VSSHcTgTtTdtllZlzmmbljTn
RqMqsFfQLLFLQFMMfRLPZLvPpCfWrbpmCbjCnfjlWmnrmmnm
hqRDqPDRsqNHwtHSNBZtJd
tNFDpDFrtdjfmjjjFmFFdScpZhZScTJgpHccHhMJgS
lLzSlSCQqbsVhBghggBZgCcJ
zRLVVLQnvQqVVzRldfWrwffjjdwSdfjv
bpWqqqWvHBpwGBCCRl
hJdjdJFQqdBBDMMC
tFFzJZFtJSqtZJQsWLbNSTnffHfvTH
lFhRZhFjPlqMlJqZJlJcRLwrLrwStRwtsVVtVSrgRV
WcpDvDfBmpDHzWBDbpbmWmNVSSTzLTtrVswgttVVzwwr
pbWfmGBpHfDmWnvvGbmWnjjMqPJMlMFPdGcjqPqPhP
NjFNRlpVLFCSSlbBWWfw
pssPZQQsMnzmtnQPttzDBbBJBcrrJWbrZSBJSbfC
QTHPHspMNGHdhvRR
QfPdSJfFJmthSthtwbsNLbPLlLTLpbvP
nHnMBnZqqgBMnWrZMqnZVcbCqRwNsvblRwppbllTsRNp
nZHBHznMnWgcrnVBtjFdfmzQNtNddjNF
hFhfPghppPhpRNhzsjsvHVzjpsGnWz
tTjlCCwMqtdMjMctGJWHwWnVwWnwvWGs
rZdrjBBtqdCtlcdgFZQLfhRLFSgRNP
RDHSWrJWffJFlJCgCMCDjCvzjPMP
QtGTndBwBtNzBVjBCMgB
LdwwMpTdwsRHsqSHqHJl
RfsfzvLLFvFzCSvSbDsTpTGMPMZPPTMt
jqWBjwBBNwWqwPGZbTwVwVtD
BnhgglhhNNngqjBjHNWrZLlFLSCJSFFCCQzQvQFCFF
HLvLDQbvnDQDvbHTLhntSnGBSlfGldddcmfMMf
NgFjZjrZZJrlfJfSVcBJGc
scWCNFZpsjzrDLwLhbQzhQwD
SlqJlThDPqpwSTwhcbDdbWDbZGcZNcDb
MsnWWjHjvLvfscjjgdzNdbgbcc
vQQvWVQFLLHfHVBWfsfmFFpJRhhSplqlRJqpBwlqTCPC
DZbDzzZDjQbPGZFFSSgSlFCzTgzm
qLnvwvhddrqMrwrCTLLFJjmtSlFlSH
VdhvsWqdVWvvRhsvqbpbPcZfPpjZGBQNRj
mJNtNFmzDZtzdzrLtwwRqJSchgfGcRfwRB
pWpjQjCTQnHMWCCpjQpHvTqcwTwScfRcBcSGBRThwS
MQHjvjVCCqsvljWnVQzLtNPZzmzLVNLddtPN
QVRPRVDgsRjLssnL
TTGDJDJfbfLHSnsMWWbs
qGqqTFFDqgQgQQQq
nlMnRRjbMjCdJVQJCZ
nGqfLwfNLFNLnPPGFVVCdVGZJtCtCCVzJz
LHHfPNHnPqqLwqPqDPWfNFvMglbhhbMgmclgcllDmgmrcl
cLLWWSThtdLpRcddcgPRZFDMCVPPMCCPCPCZ
NfGbGNzrBNffGNJjbPPZsZmZZPmDHpMH
zlJBfzlQzNjNjfJcpwSdvWhcvLwQWt
cVVQfVCJVrVcTJnfNvlDFmDrmlvrFWlL
snZHpMhZtMbtPNvzHWWvNFNvNW
gppnbbbRgMnZbswRqRwbqTcCCSTCJJdGjgfVGTdcCG
jplgNdrHrrNZgdHmlHNJHddlDSPPSTlzTSlTSDSzCQLfzf
vscvWWWvGWGGscbFMpRWFwQTPzfLQwQwPfLbzSzzDL
GvGBWpqcMVRNNZHgdHdtBJ
LchbZhjjZFjwSmPRqRffqbdtggdR
vWHMWlHJdGqtRqHV
MvzCJlnMnlTNnNNLLdhjjCdjjhDjjL
FNCllHFvCGvwQcPQJfgfmwgh
zjtRpbDLjtsrzbLLQmfBTgTBQQfhbfQB
WLgqRzqsrWvFGFZFZC
qjLlNcLjcNWpQLlQMmvmhCvCgsMZZghj
tGSDJtRGJzHMMGDVZCfvmfhzmZZgZsmv
BSSRDRHBGHtSSSbGJSwHbNcLQddqMNlrqcMQMldBWc
JSfctrtctDpszHvzVQHr
glCWjhWmFjlmlhmdWPhVVznvcHjszbvvpHvznv
FgBmFhCBCGFqglgmhCFmSTSRLJLLZfSRJcDSGMtM
vZGlFFtLMLdShSSShRVtVf
rQNvmznWPNCPNsrCsbWbsPCvjShhhfHBBHJjSJRhjSRnHhSj
mCNsQCmqszNcQzrzrrzWvGgGMgpdFpMLlFZGwcLDdg
QJRJQDlcqLlWbNGL
HCnwwsCrnstLWqtWNgZNgg
rsnTrTCHTnnVwnsVPqqDQcRjcczMPvPRzM
qCzjqnzVdzrdhnhddDbDBMPttcGBDBDPnc
sZgRQWHgWHHLsgsRRZsJbpJlDcDGNcTDFtGNFFcJNFPBPBTc
WggbRQSRRgRSsWWmbHqvVffVwhzvCdmfhmdV
lhqWcNpQGcNmmHmNPWCsQzQsgrQrBMCMbMVM
wDLFFDJvSFFZRDZSzCrzTzsRgVWbCrMW
dFwDtZfdjFZWFFfmHGPnPPmqfmPNcN
lcMRNJRGGLJnNVFbVrwrwZrD
tjCzQjQhQwgWFShVFS
ffHQsQssQTzBsPnLpMPRwsJP
MQSMSBSRFMQLJChLChjTBh
WmVlPrwnpwDlflNpDrNnDlDwThJCCdLJhhdhCfJTccGjvscd
gnDVnNnwgglwDwptSZFzgQHqbjZgZZ
nwBcFgwTDcNrpZMD
WQWCLZmvhMRvNjsNSD
CGGWmZGHHhtVzHbTqgTdbgzz
RmcTCwvssRbsThTcVRJJfSPqfJwJFqfjfMFq
zQNZDWtQlDZGBQPfFQqjJLjL
rrglggZGWnrnrrHlDhsbsPTVCsCVsTRpHv
wFGfzSvCPGttSzqwmtqmvvPRDDRCWgWWDTBTMcBcBWbCRM
hVJJHQHnpWnDTNnnDb
LJsVVdhQqvmdbbSf
srlJztzsVVsSsVtRlNllTWzzmqGhqWLPCDCgmChPLDdqCmCP
bZQMZpbvMBMgmDGmZLSPZd
MpScMSMpvfjMBcBcfMfSBnzlTjssNszrNrtlTVzlzFVN
rCtgrgClprGGClnJCZmwtMjZRjbjjcjZQv
PWVfBHWPdbNfbbRmRj
sPsVqFPsHWLhBVVqHFqPVddWSDLJgpTCnnrRRLGpJSSTRrgT
zjqpGjrQjGqSHCVvCrRZDN
cTdshMhdmcMNmddRHBhvCCBCCvHZDC
JTmTmJnLTdwzNQpPWJWgpP
BmpZmrzZnznHbpprSbQSQbqdSVqbPQcV
fRGTGJZRTTDwJTJRGDfgJgNFlSSFcldfdccFVlPlFFQPSQ
GvTTTZZLmsntzmCL
VhMcrmbhvzMSnhvftbRbllLtglBBtf
HqqqJqDqPjJPNjjDVFDZCdqBtRtGBGlGRfQQgttQfHlTQl
pCZJPqqZpmhvhpVh
dWLBJHJhGJGMBJRcDLDSQsSQpvcR
ZlnnPqglblfRRpSvSsnz
sPTgZVjjmwVTljrwTTlbwVGdJhBNNdFdMGNHHJMjBNFN
FhFrfbfgbLRdfqfrmvDgLdjrcQtSNStHHHQlSjJJPllt
CnspzZWTpCnMVzzZZGZRCzttHNjNlQlSNtNlNjVcjlQS
GCZsZBRwnvwfbqwFwb
bZnJFJgLFRnqQZqJQJFQGpCLNcGlLllClNtccjGc
rVfvwPDhPHGtlcbClr
mBhshsfMvBvqsQJdTbgnqQ
jgWHqMSWMGqWjWjqbWGJQDfVqLfrfDfJhVLfTr
pPplwsRZPFZFtLhfwgfwrhJL
zlRsdgFcRgmjdBCMHdjHWB
qJSGJSPQWzcprtQZtt
mBMVfsNBnZzcNtcc
LMLBsmMlvBgFsghVVvfgLBvbJJSqgGHqPGPtCWwbJHqCPG
ZvZLcdMGVMlHDvDpvqhH
NNSrQNbJbrTnnWZDDZqqhqpW
wbgNJrsrCwwJQZbsrJBFzjCCdzGdjcGzMdzj
JbVmdVLJJJdQMnzmmMgHjPqqjNgvqwngHNNP
ZfffDZZsRpcpRDcCRrlpplcWSSgwgSwjvvsjPSwhNSWggh
cCtfppZrpjtMMmdQQTLz
TtbnmbdmTmgTlPNhqvqj
wrwrLsVZRsJJJsfHjvPPWfhjHqRN
sDZwDvsCCQLJZQJQsMCMzZBtSMpndcSFnnSBFtSBmdBc
mWFTZdmQdZFrFQbCRsrspjSjnvCLRS
GwlDqcNHDzwGfHSRqCgJsSpnvpSL
NGlcNwHLLGfDDHDhDwDcwVczbPddZtMFWttWWtdPPdQdhPWd
mnfcZgcdZqnqdfFqPmHfhqsbgVMCJNMtvCJtMvtblTJtvb
rRLDDjPSjjPDGBQSBNbtLVtbMNNJlTMtbl
SzjDDzRRpGQDDDPHzdsmnnhsqcqdFq
ZDGNRDGjSdwnnmnsVNsHJJ
tMBWWrddLPLhvWTTPLccvmmbVpgsJHmccppJ
ClPrtBWWrhrFLBPlCRzjzGqdRzjRdRGZjF
csTRNQNJcNBDLfhfMf
qGmWpGHqrqPLChPRhVFPDD
tgHrtnrrJnZRTZcv
FLqrfmLDrqCmqjTqcbGqRTGVvb
FMtWMSWzzFStJzPzhWzhQvTvHVjjTjHTTHvbHc
PgtWWstWtSpZWPzWwnrBsdBDdFLfllLlfC
mThbMDMQDCDbwLqWpqPpdhwR
zgrcffgHNZltZSgHLsRsLLWRWgLqppsW
SVlSrfSHlSSVlrJfVctlNDMCmMFbnbRDbDBFJFbBRM
PrBrWqtRPdBLLrBwqpswgpwhgpnZhhzsgw
FTFRSVJQVJflFfQQgggGMZngGQZszZ
TbmfFJFSDFblSTDSFFbmVSDrPLLWtcmBqqRmBtmcLtcrjP
DjPsMwDjLVVTsvNNRTNTRT
ztdQQHqHlFNtfRNNNMgg
FzhMhHQlDcCrhCCc
zSHGzzmHgnnMDLTNTG
lPVBtvhQjpNSMWTLBD
VCftbjvbVCfPbZwsJsrSgSSZwC
CbwgmvMnmnCwMmwRQqJBGBgHZHpJHdtdZpJt
zVSlNSDlrzNhqlNTScDzVWfBBZZZZGBstGsdsWFpdHdJsW
NDlLzhrVcqRPCMRwLLLw
TjTHHLwnLjVlTwLjgVfvsFvDsdWfvDvFMd
qbRRRpmpcmDcczppztSqSvWFssFGfWdMvfQWdfsG
RZpqDBmtrzhzphjTgjHlnwjgJhgJ
dLmMgdgzwDLzDWFhBWvzFzzBZJ
tTVcppbSTfstTMMHfTbhBchhJFCWcjWBZhjGGB
SSSSNbsNRpRRsRrfVHfRpNtlPgQDLPdMmlDLlrPnqPdPLl
qqbTCSqdqqFZdRLZhwhZ
HWWlHtlrBfGtVssnsLnHfJVPPMMFzhPRwMPwFhzPZzPMGM
nfmtsrlsnrfVnHJrVBWlsVfgbbNTNSvmvvpcTjLjLbqvvS
GGhFvGPFcThqffPdnfNLqZZCSwtQSwZpwQQBsL
RglMRrJJgHBCBZSQQpdr
WmbRHHbzDgJMDzRDMdWmWHzHNFFvvGGhnvVvvfcvnFfcbvnT
QsfQmsLfZZZcshnJ
dSgdWgSVVFvzSpqFdqTgWRHbJNcbZNCTJCNNZRRCCh
FcpVjgDvVVFdVWFvzjwwQtBMLtBBGDwftPrB
rqsRrHsvsPqswNcJcNJrnnBrNn
bFjgGFdbVRNNnpRQpV
GSthhggGDSvMRqtHvMfM
ZwVPgMsgVsGzVsRZpgpzzgpFMrNbbLFrDLFFrrSDLfrNBN
qvnjBhQhntbfDLrF
CJlHHcHcTWqvpBdsWRpdPdgs
BjmTDjJBCBWrgQRPFlWWlW
dHphshtdtVHVhpJqspdvRrqFPgrLPPFPrrRPvQ
sdMsMtStVszpwMzHjJGjCcZjmScNfCDf
DmGdDffgDSDDdJstqdJldlRt
MhnvMCZCbbZHMvsCHtrcVrPjJcRqVtlt
LsQbsFZvZhQzZwhQWTNgBWpNwSGpTmfS
RRJQnCzbZZLTZJCBtWvFtsfqBqtfWb
prjlChGNldGNdlSVMhWfqWtfsvwvqsFtdtsq
GGjNDNhpMGMGVhrnZZTzcTHCCJcDHc
RmbMmjgpPjMBsBMfchhVsc
HwFWFTztSrtFpcQvBsSqVscBBC
zWwnJFHtWWHDgbGgdpGpnl
mnbWbRRLRFnmmWcCDTBVwCDBlwNW
ggJPtpdHGfdZtMHgtZgVPPBCVsPNBcsBTTDDCC
hpvJJTpGhdhtJdMHqvmmnLvSbmnFnRFm
WWtrWrNgVbRjMrQCNzqJFwQJFNTJ
LdHPhcdchQQssLzJrz
pBccnHpnrrcGHnnSlWjnRMSlbt
NMMfNFnZgMVThhTMcgTDJDJjsVvvJJqJmHsqHG
LQpwwprCQzBNBdGjGjHswswdvm
CBCzzCrbWbSlNQnTRgPPfFRWnfgc
RFwHVQRwFgTQSFVhdsdHsBdDBnnqnq
LGftLtPGGMzlNrhlPqPsrJ
fvGpWpMtccpTwwpRRQhh
TTJCGdTGtZRQQCnzcnCv
FWWHPSFNFbDbDDqSWnVmLRRjRRQLhcmLjS
qPwPWwFppbwggGZGfdJZgdnGdd
zSTWzrzWTLWpCtCGpqqGgplc
nZWwsJVZZBnJHJCclHllgtChgCgc
DFnVBJsFssVVFBFnBdfvjDSmTMWzrmMfRmTv
MJmgMssrsggqqMVstbwTcTbPbTTwThmw
NRBBGRjHVRRcRbCp
QnSfzLWzNHzNVQQVjrglJMsMFvgJdFWrgZ
ggLLGnhgnPvJHZnN
VBtmVSldbSBVlcNPHvjmNcwNZZ
tdWqSVSSBztVWGrThLhfrfvG
TDqrjdSwLqDppdTCdzPBFmmjQmhHFPFQhPFR
zlGbMcVcVtsPHFRhWRRsPF
btgvlVVcDZZZqgrz
DgwlgbbFDDjjPTHDrmddPhPV
WqtMBBtQsttMNWQBqsbJpGGzdPdTHLVmTzJhmTPhHHPTmH
qQsqGZNQtZGMNsNtZpFnjnCRbZffwwSRljFf
gMdFLCdnMZCTFFCqnTgWLCHfSgPgPHStcQQmfSBBSfHg
vrwwrwzbGjjswjvhGGsjPQmqRmHPbBtcBQtqfmcH
qzJllVsGVGljjsrzwDzhwzDGTddNLFnZWNdpCVWTNTZTLZCF
LtwMhDtctwbwwppdWBJQJBWPvPfDfqvG
FTzrNrgSRFrgzFRHNVFQJvlqHjBvQWlQWqPBfq
sFgNzmVmNzgTvVTMwhMhstMwZtsbsc
MrBDQVzzlrvhQzQrDMVQrzrzgRJnRRwwRbwSwwVRRNSgwwwJ
qFTPTvfTHcqqncpcwR
LmtdGGPmTPGCTLHLWsZMhvZMMMzrzzdlMQ
ZVNpjfpZNpfNgNjzNVfWtnbbWmBHtsZWBSZBGS
MrDrQvvDrPLDMvFvdmBGGsBBCtsHrnrGCm
ltRMwLLDDRlvQwvlQcwhqfcJNpgzjJpjhJ
sRRRlRbcFbBBdnFBwCGppNvGrTCDDGVNlr
PPSLQzHjzZZPLZPjgTNTgpCbVJvGrNCTGr
ZLHHPQjhQmWWSRRnssdtbnmfwF
GRwrMrHJGwJPGWsgfqQgsc
VbTvLQCZLSWWsgWf
TVDvVCvppvTDmzZVTbZpTzBBNQQQJlJBBJBNNJmRBwRH
shJRWJsjZGNjSTrjFS
dMLCddggldQzMCCVgzVVLmLvTwNFFSqpNSqSbFGSqTTpMTFN
VGQvVglCLcVzgdddCDVvlsPZRRBDJPHZWZZnBsWJRR
CrwlwhRCMrswnsHBFccHHWFc
QJTmtfQgLtzQfLQfdPcWSFHHDDSpcFpFBg
jTQTqbfQfmLbLQJbJrRCWjljZGjNrZlZlC
JmthDmLShtJmHphphJQCwjdjdFDzFgzFdgdNlC
sbMTVBrWMbNvVMnsWMnVzjsjwCfjFgfZzfdgdzlj
NvqbbBcMMPPSqLSpGGthmp
RfGWFHlPFFNWGFZRZBjvwCvzBwhhrvvjzmrr
sLJSLMSTSJTbStJtMSqSqbpMrvmrzWdvhmjDCzzwrrpjdDDv
SbQqsqsWcZPcQGFG
BjqbMqMVBsfqGqFqGLmF
ZZQbQPddPcwbPnRQltdtQZdnmFNrvfhGrhrWWFNWWtmNFNNW
dJJQccnRPpcbQcMHsSgSMsDMTJSg
WWGBBvPflnWbBWhvhbPvNfnnVCFZmVRVZmVGMVwRLCCCGwVC
gjszgTMrgzgqCRRdmJRjJLVw
grzQHzqczMSzqSHcgQsqPvPlbNblpPhhPPbHvnhp
sJDDNWdnRLTTvqwSFPCmLCCrCq
thzplgfjglflFcbMclpppMfcwPqCZQCmqCwrzCqmQmHSqPqq
MhcpFBMBlhjbBTdnNJWvNvsvBd
czwwghnWWfcfgwfWthfrvVvrjdrdvDDVrbzrLF
RHPPMRpQPRMPPJRjJQsZsrrvvJBDDVDVdFqrBrFdBv
smjMsGZHRsHSmRQNGHPpSTwwttCflwngnChcCtWW
bprrrwrtLDtrWwrQjRDQDbPPVHVmmmmHNWlPlVNPZZlv
hqqhfnBCTfnnhzJwzsqzfPZZMCCVZVHHFvZMFvZmlC
TzhhdJTqJzcBdJJnzjtQrLdjwgLtpbgrLQ
qzQvzzgWSCqtqqGpddGc
jLrZNZhZrNRLHNffhrjNjNdtdZtGcPFwFwpbGwbVpdwC
nHnhrLNCCMHmhHBMhrzvgJvsWSWMWzzWzSlv
RzcbzdRFzbbzbzbFdZFTHMZPhVhVQMLrlrQPhLZlMM
BNGfBvsNttVmMhlMLm
BwGjpllswfjwpcFDWcWcbpdb
SjzpswrLSDjVSpwlmZJBTBdNJLvBNvHQZT
rCcCtbqgCfthggtbGGMqqghqZQvvQTBNJQHQZQTcZTJFZFFd
CggGMtqMfWbbGghPhhbCMtmsSppSspjpmWzjVSWlVrrm
PmWTPThTQWnLWQFl
VNcSVfMbtsddBQNnNpdl
sSjctwjVSzzccjgnTnDTHRDhqjRR
WfMWfCNCjWWHNTccMjRjfRcMbqSwfVwqwsfGGbssrJSrswVw
llLFQLlvlPFnhQBPBZQBqvBwzSzGGhShJVwShmsJbbmzSG
lnPqvQZBFFBnnpgplFvtvHDjTdcTjTMMjCRNCMWgRC
rprFNFFNjNLmMdgcqL
BvzCQQbBQgffsDbvVHMdbcVqmLVqlmqq
JvJCzBDJwnsRnQDszCBnnnQBrjZPjFpgZFTFZRpTrpZFGFtT
wBHQQZHVCcpwDgdZdMsZjvMZFn
GPSzlNlJLfzzzvsWdWLMmFWLMM
NfqGSfrTNzRTqJfRbptQHFQFrwrFHBHw
sNjVMVNVMzPzQgghcMsNzJtjSJtTFDTJtJnnDLjDnL
CHwrdCpvCrwrWdpZqcpFttJSFJTLLHLJfbnbfD
qrlZCwlqZrqqpWdlRqCRqdqcVNsVMzQzmNgNPBsRhVQVVzMs
";

fn part1() {
    let mut result = Vec::new();
    for rucksack in INPUT_PART1.split("\n") {
        let middle = rucksack.len() / 2;
        let (comp1, comp2) = (&rucksack[..middle], &rucksack[middle..]);
        for item in comp1.chars() {
            if comp2.contains(&item.to_string()) {
                result.push(item);
                break;
            }
        }
    }
    println!("chars: {}", result.iter().collect::<String>());
    let sum = result.into_iter().map(|l| {
        let c = l as u32;
        if c >= ('a' as u32) && c <= ('z' as u32) {
            1 + c - ('a' as u32)
        }
        else {
            27 + c - ('A' as u32)
        }
    }).sum::<u32>();
    println!("priority sum: {}", sum);
}

fn part2() {
    let mut result = Vec::new();
    for [rucksack1, rucksack2, rucksack3] in INPUT_PART1.split("\n").into_iter().array_chunks::<3>() {
        for item in rucksack1.chars() {
            if rucksack2.contains(&item.to_string()) && rucksack3.contains(&item.to_string()) {
                result.push(item);
                break;
            }
        }
    }
    println!("chars: {}", result.iter().collect::<String>());
    let sum = result.into_iter().map(|l| {
        let c = l as u32;
        if c >= ('a' as u32) && c <= ('z' as u32) {
            1 + c - ('a' as u32)
        }
        else {
            27 + c - ('A' as u32)
        }
    }).sum::<u32>();
    println!("priority sum: {}", sum);
}

fn main() {
    println!("part1");
    part1();
    println!("part2");
    part2();
}
