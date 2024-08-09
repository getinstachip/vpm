module memory_dp # (parameter DW    = 104,      //memory width
		       parameter DEPTH = 32,       //memory depth
		       parameter PROJ  = "",       //project name
		       parameter MCW   = 8,         //repair/config vector width
		       parameter AW    = $clog2(DEPTH) // address bus width
		       ) 
   (// Memory interface (dual port)
    input 	    wr_clk, //write clock
    input 	    wr_en, //write enable
    input [DW-1:0]  wr_wem, //per bit write enable
    input [AW-1:0]  wr_addr,//write address
    input [DW-1:0]  wr_din, //write data
    input 	    rd_clk, //read clock
    input 	    rd_en, //read enable
    input [AW-1:0]  rd_addr,//read address
    output [DW-1:0] rd_dout,//read output data
    );
   
   generate
	begin : generic
	   memory_ram #(.DW(DW),
			   .DEPTH(DEPTH))	     
	   memory_dp (//read port
		      .rd_dout	(rd_dout[DW-1:0]),
		      .rd_clk	(rd_clk),
		      .rd_en	(rd_en),
		      .rd_addr	(rd_addr[AW-1:0]),
		      //write port
		      .wr_en	(wr_en),
		      .wr_clk	(wr_clk),
		      .wr_addr	(wr_addr[AW-1:0]),
		      .wr_wem	(wr_wem[DW-1:0]),
		      .wr_din	(wr_din[DW-1:0]));
	end
   endgenerate
      
endmodule // memory_dp