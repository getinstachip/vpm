module missRegister(input reset,input [23:0] in1, input [2:0] in2, input miss, output reg out);
	always@(in1,in2,reset)
		begin
		  if(reset)
		    out=1'b0;
		  else
		    out=miss;
		end
endmodule