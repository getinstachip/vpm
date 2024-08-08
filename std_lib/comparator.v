module comparator(input [23:0] in1, input [23:0] in2,output reg compOut);
  always@(in1,in2)
  if(in1==in2) begin compOut=1'b1;end
  else begin compOut=1'b0;end
endmodule