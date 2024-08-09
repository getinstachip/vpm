module rsync #(parameter PS = 2          // number of sync stages
		  )
   (
    input  clk,
    input  nrst_in,
    output nrst_out
    );

   generate
	begin :g0
	   reg [PS-1:0] sync_pipe;   
	   always @ (posedge clk or negedge nrst_in)		 
	     if(!nrst_in)
	       sync_pipe[PS-1:0] <= 1'b0;
	     else
	       sync_pipe[PS-1:0] <= {sync_pipe[PS-2:0],1'b1};	      	      
	   assign nrst_out = sync_pipe[PS-1];
	end
   endgenerate
      
endmodule // rsync