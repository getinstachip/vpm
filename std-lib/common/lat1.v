module lat1 #(parameter DW    = 1         //data width
		 )
   ( input 	     clk, // clk, latch when clk=1
     input [DW-1:0]  in,  // input data
     output [DW-1:0] out  // output data (stable/latched when clk=0)
     );

   generate
	begin : g0
	   reg [DW-1:0] out_reg;	   
	   always @ (clk or in)
	     if (clk)
	       out_reg[DW-1:0] <= in[DW-1:0];
	   assign out[DW-1:0] = out_reg[DW-1:0];	   
	end
   endgenerate
   
endmodule // lat1