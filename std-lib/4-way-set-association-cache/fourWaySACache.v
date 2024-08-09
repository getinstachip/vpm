module fourWaySACache(input clk, input reset, input [1:0] way, input replace, input [23:0] tag, input [2:0] index, output iHit, output [1:0] wayOut, output [3:0] validComp);
  wire hit,set0,set1,cmpOut0,cmpOut1,cmpOut2,cmpOut3,
  validOut00,validOut01,validOut02,validOut03,validOut04,validOut05,validOut06,validOut07,
  validOut10,validOut11,validOut12,validOut13,validOut14,validOut15,validOut16,validOut17,
  validOut20,validOut21,validOut22,validOut23,validOut24,validOut25,validOut26,validOut27,
  validOut30,validOut31,validOut32,validOut33,validOut34,validOut35,validOut36,validOut37;
  
  wire [3:0] wyOt;
  wire [7:0] dcOt, wrEn0,wrEn1,wrEn2,wrEn3;
  
  wire [23:0] tagComp0,tagComp1,tagComp2,tagComp3,
  tagOut00,tagOut01,tagOut02,tagOut03,tagOut04,tagOut05,tagOut06,tagOut07,
  tagOut10,tagOut11,tagOut12,tagOut13,tagOut14,tagOut15,tagOut16,tagOut17,
  tagOut20,tagOut21,tagOut22,tagOut23,tagOut24,tagOut25,tagOut26,tagOut27,
  tagOut30,tagOut31,tagOut32,tagOut33,tagOut34,tagOut35,tagOut36,tagOut37;
  
  decoder3to8 ind(index, dcOt);
  decoder2to4 wydc(way, wyOt);
  and an00(wrEn0[0],dcOt[0],wyOt[0]&replace);and an01(wrEn0[1],dcOt[1],wyOt[0]&replace);and an02(wrEn0[2],dcOt[2],wyOt[0]&replace);and an03(wrEn0[3],dcOt[3],wyOt[0]&replace);
  and an04(wrEn0[4],dcOt[4],wyOt[0]&replace);and an05(wrEn0[5],dcOt[5],wyOt[0]&replace);and an06(wrEn0[6],dcOt[6],wyOt[0]&replace);and an07(wrEn0[7],dcOt[7],wyOt[0]&replace);
  and an10(wrEn1[0],dcOt[0],wyOt[1]&replace);and an11(wrEn1[1],dcOt[1],wyOt[1]&replace);and an12(wrEn1[2],dcOt[2],wyOt[1]&replace);and an13(wrEn1[3],dcOt[3],wyOt[1]&replace);
  and an14(wrEn1[4],dcOt[4],wyOt[1]&replace);and an15(wrEn1[5],dcOt[5],wyOt[1]&replace);and an16(wrEn1[6],dcOt[6],wyOt[1]&replace);and an17(wrEn1[7],dcOt[7],wyOt[1]&replace);
  and an20(wrEn2[0],dcOt[0],wyOt[2]&replace);and an21(wrEn2[1],dcOt[1],wyOt[2]&replace);and an22(wrEn2[2],dcOt[2],wyOt[2]&replace);and an23(wrEn2[3],dcOt[3],wyOt[2]&replace);
  and an24(wrEn2[4],dcOt[4],wyOt[2]&replace);and an25(wrEn2[5],dcOt[5],wyOt[2]&replace);and an26(wrEn2[6],dcOt[6],wyOt[2]&replace);and an27(wrEn2[7],dcOt[7],wyOt[2]&replace);
  and an30(wrEn3[0],dcOt[0],wyOt[3]&replace);and an31(wrEn3[1],dcOt[1],wyOt[3]&replace);and an32(wrEn3[2],dcOt[2],wyOt[3]&replace);and an33(wrEn3[3],dcOt[3],wyOt[3]&replace);
  and an34(wrEn3[4],dcOt[4],wyOt[3]&replace);and an35(wrEn3[5],dcOt[5],wyOt[3]&replace);and an36(wrEn3[6],dcOt[6],wyOt[3]&replace);and an37(wrEn3[7],dcOt[7],wyOt[3]&replace);
  vaildArray vdAr0(clk,reset,wrEn0,1'b1,validOut00,validOut01,validOut02,validOut03,validOut04,validOut05,validOut06,validOut07);
  vaildArray vdAr1(clk,reset,wrEn1,1'b1,validOut10,validOut11,validOut12,validOut13,validOut14,validOut15,validOut16,validOut17);
  vaildArray vdAr2(clk,reset,wrEn2,1'b1,validOut20,validOut21,validOut22,validOut23,validOut24,validOut25,validOut26,validOut27);
  vaildArray vdAr3(clk,reset,wrEn3,1'b1,validOut30,validOut31,validOut32,validOut33,validOut34,validOut35,validOut36,validOut37);
  tagArray tgAr0(clk,reset,wrEn0,tag,tagOut00,tagOut01,tagOut02,tagOut03,tagOut04,tagOut05,tagOut06,tagOut07);
  tagArray tgAr1(clk,reset,wrEn1,tag,tagOut10,tagOut11,tagOut12,tagOut13,tagOut14,tagOut15,tagOut16,tagOut17);
  tagArray tgAr2(clk,reset,wrEn2,tag,tagOut20,tagOut21,tagOut22,tagOut23,tagOut24,tagOut25,tagOut26,tagOut27);
  tagArray tgAr3(clk,reset,wrEn3,tag,tagOut30,tagOut31,tagOut32,tagOut33,tagOut34,tagOut35,tagOut36,tagOut37);
  mux8to1_24b mx24b0(tagOut00,tagOut01,tagOut02,tagOut03,tagOut04,tagOut05,tagOut06,tagOut07,index,tagComp0);
  mux8to1_24b mx24b1(tagOut10,tagOut11,tagOut12,tagOut13,tagOut14,tagOut15,tagOut16,tagOut17,index,tagComp1);
  mux8to1_24b mx24b2(tagOut20,tagOut21,tagOut22,tagOut23,tagOut24,tagOut25,tagOut26,tagOut27,index,tagComp2);
  mux8to1_24b mx24b3(tagOut30,tagOut31,tagOut32,tagOut33,tagOut34,tagOut35,tagOut36,tagOut37,index,tagComp3);
  mux8to1_1b mx1b0(validOut00,validOut01,validOut02,validOut03,validOut04,validOut05,validOut06,validOut07,index,validComp[0]);
  mux8to1_1b mx1b1(validOut10,validOut11,validOut12,validOut13,validOut14,validOut15,validOut16,validOut17,index,validComp[1]);
  mux8to1_1b mx1b2(validOut20,validOut21,validOut22,validOut23,validOut24,validOut25,validOut26,validOut27,index,validComp[2]);
  mux8to1_1b mx1b3(validOut30,validOut31,validOut32,validOut33,validOut34,validOut35,validOut36,validOut37,index,validComp[3]);
  comparator cmp0(tag,tagComp0,cmpOut0);
  comparator cmp1(tag,tagComp1,cmpOut1);
  comparator cmp2(tag,tagComp2,cmpOut2);
  comparator cmp3(tag,tagComp3,cmpOut3);
  encoder4to2 encodeWay({{cmpOut3 & validComp[3]},{cmpOut2 & validComp[2]},{cmpOut1 & validComp[1]},{cmpOut0 & validComp[0]}}, wayOut);
  or in0(set0,cmpOut0 & validComp[0],cmpOut1 & validComp[1]);
  or in1(set1,cmpOut2 & validComp[2],cmpOut3 & validComp[3]);
  or fin(iHit, set0, set1);
  missRegister hitReg(reset,tag,index, iHit, hit);
endmodule