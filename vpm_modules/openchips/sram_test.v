module SramUnit #(
  parameter NUM_WMASKS = 4,
  parameter DATA_WIDTH = 32,
  parameter ADDR_WIDTH = 8
)(
  input clk,  
  input rst_n,   
  input csb0,  
  input csb1,     
  input web0,    
  input [NUM_WMASKS-1:0] wmask0, 
  input [ADDR_WIDTH-1:0] addr0,   
  input [DATA_WIDTH-1:0] din0,   
  output [DATA_WIDTH-1:0] dout0,
  output [DATA_WIDTH-1:0] dout1  
);

  reg [ADDR_WIDTH-1:0] addr1;  

  // Instantiate the SRAM
  sky130_sram_1kbyte_1rw1r_32x256_8 sram(
    .clk0(clk),
    .csb0(csb0),
    .web0(web0),
    .wmask0(wmask0),
    .addr0(addr0),
    .din0(din0),
    .dout0(dout0),
    .clk1(clk),
    .csb1(csb1),
    .addr1(addr1),
    .dout1(dout1)
  );

  // Address counter for port 1
  always @ (posedge clk or negedge rst_n) begin
    if (!rst_n) begin
      addr1 <= 0;  
    end else if (!csb1) begin
      addr1 <= addr1 + 1;  // Increment address counter when csb1 is low
    end
  end

endmodule
