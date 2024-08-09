module data_cache (
    input wire clk,
    input wire reset,
    input wire [31:0] address,  // Memory address
    input wire [31:0] write_data, // Data to write
    input wire mem_read,        // Read enable
    input wire mem_write,       // Write enable
    output reg [31:0] read_data, // Data read from cache
    output reg hit              // Cache hit indicator
);

    // Cache parameters
    parameter CACHE_SIZE = 64;  // Number of cache lines
    parameter LINE_SIZE = 32;   // Size of each cache line in bits

    // Cache storage
    reg [LINE_SIZE-1:0] cache_mem [0:CACHE_SIZE-1];
    reg [31:0] tag_array [0:CACHE_SIZE-1];
    reg valid_array [0:CACHE_SIZE-1];

    integer i;

    // Cache operation
    always @(posedge clk or posedge reset) begin
        if (reset) begin
            // Initialize cache on reset
            for (i = 0; i < CACHE_SIZE; i = i + 1) begin
                valid_array[i] <= 0;
                tag_array[i] <= 0;
            end
        end else begin
            // Calculate cache index and tag
            integer index = address[5:0]; // Assuming 6-bit index for 64 lines
            integer tag = address[31:6];

            if (mem_read) begin
                if (valid_array[index] && (tag_array[index] == tag)) begin
                    // Cache hit
                    read_data <= cache_mem[index];
                    hit <= 1;
                end else begin
                    // Cache miss
                    read_data <= 32'hDEADBEEF; // Placeholder for miss
                    hit <= 0;
                end
            end

            if (mem_write) begin
                // Write data to cache
                cache_mem[index] <= write_data;
                tag_array[index] <= tag;
                valid_array[index] <= 1;
            end
        end
    end

endmodule
