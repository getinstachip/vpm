module dsync  #(parameter PS    = 2,        // number of sync stages
		   parameter DELAY = 0        // random delay
		   )
   (
    input  clk, // clock
    input  nreset, // clock
    input  din, // input data
    output dout    // synchronized data
    );

   generate
	begin : g0
	   reg [PS:0]   sync_pipe; 
	   always @ (posedge clk or negedge nreset)		 
	     if(!nreset)
	       sync_pipe[PS:0] <= 1'b0;
	     else
	       sync_pipe[PS:0] <= {sync_pipe[PS-1:0],din};	      	      
	   // drive randomize delay from testbench
	   assign dout = (DELAY & sync_pipe[PS]) |  //extra cycle
			 (~DELAY & sync_pipe[PS-1]); //default
      	end 
   endgenerate
   
endmodule // dsync