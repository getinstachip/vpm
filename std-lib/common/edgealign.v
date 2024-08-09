module edgealign (/*AUTOARG*/
   // Outputs
   firstedge,
   // Inputs
   fastclk, slowclk
   );
   
   input  fastclk;
   input  slowclk;
   output firstedge;

   reg 	  clk45;
   reg 	  clk90;
   reg 	  firstedge;

   always @ (negedge fastclk) 
     clk45     <= slowclk;

   always @ (posedge fastclk) 
     begin
	clk90     <= clk45;
	firstedge <= ~clk45 & clk90;
     end
   
endmodule // edgealign