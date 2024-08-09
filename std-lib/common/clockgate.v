module clockgate (
     input  clk, // clock input 
     input  te, // test enable enable   
     input  en, // enable (from positive edge FF)
     output eclk // enabled clock output
     );

   generate
	begin : generic
	   wire    en_sh;
	   wire    en_sl;
	   //Stable low/valid rising edge enable
	   assign   en_sl = en | te;
	   //Stable high enable signal
	   lat0 lat0 (.out (en_sh),
			 .in  (en_sl),
			 .clk (clk));
	   
	   assign eclk =  clk & en_sh;
	end 
   endgenerate
        
endmodule