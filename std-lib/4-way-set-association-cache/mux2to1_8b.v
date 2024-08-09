module mux2to1_8b(input [7:0] zero, input [7:0] inpt, input dataRead, output reg [7:0] dataOut);	
	always@(inpt or zero or dataRead)
		case (dataRead)
			1'b0: dataOut=zero;
			1'b1: dataOut=inpt;
		endcase	
endmodule