module mux8to1_2b(input [1:0]in1, input [1:0]in2, input [1:0]in3, input [1:0]in4, input [1:0]in5, input [1:0]in6, input [1:0]in7, input [1:0]in8,input[2:0] sel,output reg [1:0]priorityOut);
  always@(in1,in2,in3,in4,in5,in6,in7,in8,sel)
  case(sel)
    3'b000: priorityOut=in1;
    3'b001: priorityOut=in2;
    3'b010: priorityOut=in3;
    3'b011: priorityOut=in4;
    3'b100: priorityOut=in5;
    3'b101: priorityOut=in6;
    3'b110: priorityOut=in7;
    3'b111: priorityOut=in8;
  endcase
endmodule