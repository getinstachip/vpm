module validityEncoder(input [3:0] validBits,output reg invalidReplace, output reg [1:0] out); 
  always @ (validBits)
  begin  
  invalidReplace=1'b0;
  if(validBits[0]==0) out=2'b00;
  else if(validBits[1]==0)  out=2'b01;  
  else if(validBits[2]==0)  out=2'b10; 
  else if(validBits[3]==0)  out=2'b11;
  else invalidReplace=1'b1; 
  end
endmodule