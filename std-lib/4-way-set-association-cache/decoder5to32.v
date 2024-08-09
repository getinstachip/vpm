module decoder5to32(input [4:0] in,output reg [31:0] decoder_out);
  always @ (in)
    case (in)
      5'h00 : decoder_out = 32'h00000001;
      5'h01 : decoder_out = 32'h00000002;
      5'h02 : decoder_out = 32'h00000004;
      5'h03 : decoder_out = 32'h00000008;
      5'h04 : decoder_out = 32'h00000010;
      5'h05 : decoder_out = 32'h00000020;
      5'h06 : decoder_out = 32'h00000040;
      5'h07 : decoder_out = 32'h00000080;
      5'h08 : decoder_out = 32'h00000100;
      5'h09 : decoder_out = 32'h00000200;
      5'h0A : decoder_out = 32'h00000400;
      5'h0B : decoder_out = 32'h00000800;
      5'h0C : decoder_out = 32'h00001000;
      5'h0D : decoder_out = 32'h00002000;
      5'h0E : decoder_out = 32'h00004000;
      5'h0F : decoder_out = 32'h00008000;
      5'h10 : decoder_out = 32'h00010000;
      5'h11 : decoder_out = 32'h00020000;
      5'h12 : decoder_out = 32'h00040000;
      5'h13 : decoder_out = 32'h00080000;
      5'h14 : decoder_out = 32'h00100000;
      5'h15 : decoder_out = 32'h00200000;
      5'h16 : decoder_out = 32'h00400000;
      5'h17 : decoder_out = 32'h00800000;
      5'h18 : decoder_out = 32'h01000000;
      5'h19 : decoder_out = 32'h02000000;
      5'h1A : decoder_out = 32'h04000000;
      5'h1B : decoder_out = 32'h08000000;
      5'h1C : decoder_out = 32'h10000000;
      5'h1D : decoder_out = 32'h20000000;
      5'h1E : decoder_out = 32'h40000000;
      5'h1F : decoder_out = 32'h80000000;
    endcase
endmodule