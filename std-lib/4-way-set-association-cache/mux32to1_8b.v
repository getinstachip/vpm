module mux32to1_8b(input [7:0] in01,input [7:0] in02, input [7:0] in03, input [7:0] in04, input [7:0] in05,input [7:0] in06, input [7:0] in07, input [7:0] in08,
  input [7:0] in09,input [7:0] in10, input [7:0] in11, input [7:0] in12, input [7:0] in13,input [7:0] in14, input [7:0] in15, input [7:0] in16,
  input [7:0] in17,input [7:0] in18, input [7:0] in19, input [7:0] in20, input [7:0] in21,input [7:0] in22, input [7:0] in23, input [7:0] in24,
  input [7:0] in25,input [7:0] in26, input [7:0] in27, input [7:0] in28, input [7:0] in29,input [7:0] in30, input [7:0] in31, input [7:0] in32,
 input[4:0] sel,output reg [7:0] muxOut);
  always@(in01,in02,in03,in04,in05,in06,in07,in08,in09,in10,in11,in12,in13,in14,in15,in16,in17,in18,in19,in20,in21,in22,in23,in24,in25,in26,in27,in28,in29,in30,in31,in32,sel)
  case(sel)
    5'b00000: muxOut=in01;5'b00001: muxOut=in02;5'b00010: muxOut=in03;5'b00011: muxOut=in04;
    5'b00100: muxOut=in05;5'b00101: muxOut=in06;5'b00110: muxOut=in07;5'b00111: muxOut=in08;
    5'b01000: muxOut=in09;5'b01001: muxOut=in10;5'b01010: muxOut=in11;5'b01011: muxOut=in12;
    5'b01100: muxOut=in13;5'b01101: muxOut=in14;5'b01110: muxOut=in15;5'b01111: muxOut=in16;
    5'b10000: muxOut=in17;5'b10001: muxOut=in18;5'b10010: muxOut=in19;5'b10011: muxOut=in20;
    5'b10100: muxOut=in21;5'b10101: muxOut=in22;5'b10110: muxOut=in23;5'b10111: muxOut=in24;
    5'b11000: muxOut=in25;5'b11001: muxOut=in26;5'b11010: muxOut=in27;5'b11011: muxOut=in28;
    5'b11100: muxOut=in29;5'b11101: muxOut=in30;5'b11110: muxOut=in31;5'b11111: muxOut=in32;
  endcase
endmodule