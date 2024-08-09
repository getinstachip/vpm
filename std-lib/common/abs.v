module abs #(parameter DW = 2) // data width
   (
    input [DW-1:0]  in, //input operand
    output [DW-1:0] out, //out = abs(in) (signed two's complement)
    output 	    overflow  //high for max negative #
    );
   
   assign out[DW-1:0] = in[DW-1] ? ~in[DW-1:0] + 1'b1 :
			            in[DW-1:0];

   assign overflow = in[DW-1] & ~(|in[DW-2:0]);

endmodule