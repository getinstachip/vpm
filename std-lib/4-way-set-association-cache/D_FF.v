module D_FF(input clk, input reset, input write, input d, output reg q);
  always @(negedge clk)
  if(reset) q=0;
  else
    if(write) q=d;
endmodule