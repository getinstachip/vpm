module CtrlCkt( input clk, input reset, input write, input hit, output reg replace,output reg finalWrite,output reg finalRead,output reg bufferWrite,output reg sel);
	reg [3:0] state,next_state;
	
	always @ (negedge clk or reset)
	begin
		if(reset==0) state=next_state;
		else begin state=0; next_state=0; end
	end
	
	always @ (state or reset)
	begin	
	  if(reset==0)
		case(state)
			0:begin 
					replace=0;
					finalWrite=0;bufferWrite=0;
					sel=0;
					finalRead=0;
					if(hit && ~write) next_state=1;                     //read hit
					else if(hit && write) next_state=3;                 //write hit
					else next_state=2;
				end
			1:begin 
					replace=0;
					finalWrite=0;
					if(write)  
					   bufferWrite=1;
					else       
					   bufferWrite=0;       
					sel=0;
					finalRead=1;
					next_state=0;
				end
			2:begin 
					replace=1;
					finalWrite=write;bufferWrite=0;
					sel=0;
					finalRead=0;
					if(write)  next_state=3;           //write miss
					else       next_state=1;           //read miss
				end
			3:begin
			    replace=0;
			    finalWrite=1;bufferWrite=0;	
			    sel=1;
			    finalRead=1;
			    next_state=1;
			  end
		endcase			
	end	
endmodule