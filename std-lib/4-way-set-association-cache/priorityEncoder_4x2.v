module priorityEncoder_4x2(input [3:0] in,output reg [1:0] out);
  always @ (in)
  begin
    if(in[3]==0) out=2'b11;
    else if(in[2]==0) out=2'b10;
    else if(in[1]==0) out=2'b01;
    else if(in[0]==0) out=2'b00;   
  end
endmodule