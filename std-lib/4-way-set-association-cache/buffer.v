module buffer(input clock, input reset, input write_en, input [26:0] addr_in, input [255:0] data_in);
	reg [283:0] reg_array [0:3];
	reg [1:0] write_ptr;
	always @(posedge clock,reset)
	begin
	  if(reset)
	    write_ptr=2'b00;
	  else
	    begin
      if (write_en)     
		    begin
			  reg_array[write_ptr] = {data_in, addr_in, 1'b1};
		    if(write_ptr==2'b11)  
		      write_ptr = 2'b00;
		    else                        //BUFFER REACHED LAST BLOCK
		      write_ptr=write_ptr+1;
		    end
		  end
	end
endmodule