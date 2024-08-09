// Memory Management Unit
module memory_management_unit (
    input wire [31:0] virtual_address, // Virtual address
    output reg [31:0] physical_address // Physical address
);

    // Simple page table for demonstration
    reg [31:0] page_table [0:15]; // Assume 16 pages

    initial begin
        // Initialize page table with identity mapping for simplicity
        integer i;
        for (i = 0; i < 16; i = i + 1) begin
            page_table[i] = i << 12; // Example: page size of 4KB
        end
    end

    always @(*) begin
        // Extract page number and offset
        integer page_number = virtual_address[31:12];
        integer offset = virtual_address[11:0];

        // Translate virtual address to physical address
        physical_address = page_table[page_number] | offset;
    end

endmodule
