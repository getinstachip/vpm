module memory_sp  # (parameter DW    = 104,          // memory width
			parameter DEPTH = 32,           // memory depth
			parameter MCW   = 8,            // repair/config width
			parameter AW    = $clog2(DEPTH) // address bus width  
		       ) 
   (// memory interface (single port)
    input 	    clk, // clock
    input 	    en, // memory access   
    input 	    we, // write enable global signal   
    input [DW-1:0]  wem, // write enable vector
    input [AW-1:0]  addr, // address
    input [DW-1:0]  din, // data input
    output [DW-1:0] dout, // data output
    );

   generate
	begin : g0
	   memory_ram #(.DW(DW),
			   .DEPTH(DEPTH))	     
	   sram_sp (//read port
		    .rd_dout (dout[DW-1:0]),
		    .rd_clk  (clk),
		    .rd_addr (addr[AW-1:0]),
		    .rd_en   (en & ~we),
		    //write port
		    .wr_clk  (clk),
		    .wr_en   (en & we),
		    .wr_addr (addr[AW-1:0]),
		    .wr_wem  (wem[DW-1:0]),
		    .wr_din  (din[DW-1:0]));
	end
   endgenerate
  
endmodule // memory_sp