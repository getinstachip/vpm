module cache #(
    parameter ADDR_WIDTH = 32,
    parameter DATA_WIDTH = 64,
    parameter CACHE_SIZE = 1024,  // in bytes
    parameter LINE_SIZE = 64,     // in bytes
    parameter WAYS = 4
) (
    input wire clk,
    input wire rst_n,
    
    // CPU interface
    input wire [ADDR_WIDTH-1:0] cpu_addr,
    input wire [DATA_WIDTH-1:0] cpu_data_in,
    output reg [DATA_WIDTH-1:0] cpu_data_out,
    input wire cpu_read,
    input wire cpu_write,
    output reg cpu_ready,
    
    // Memory interface
    output reg [ADDR_WIDTH-1:0] mem_addr,
    input wire [DATA_WIDTH-1:0] mem_data_in,
    output reg [DATA_WIDTH-1:0] mem_data_out,
    output reg mem_read,
    output reg mem_write,
    input wire mem_ready
);

    localparam OFFSET_BITS = $clog2(LINE_SIZE);
    localparam INDEX_BITS = $clog2(CACHE_SIZE / LINE_SIZE / WAYS);
    localparam TAG_BITS = ADDR_WIDTH - INDEX_BITS - OFFSET_BITS;

    reg [DATA_WIDTH-1:0] cache_data [WAYS-1:0][CACHE_SIZE/LINE_SIZE/WAYS-1:0][LINE_SIZE/DATA_WIDTH-1:0];
    reg [TAG_BITS-1:0] cache_tags [WAYS-1:0][CACHE_SIZE/LINE_SIZE/WAYS-1:0];
    reg cache_valid [WAYS-1:0][CACHE_SIZE/LINE_SIZE/WAYS-1:0];
    reg [WAYS-1:0] lru_bits [CACHE_SIZE/LINE_SIZE/WAYS-1:0];

    wire [TAG_BITS-1:0] tag = cpu_addr[ADDR_WIDTH-1:ADDR_WIDTH-TAG_BITS];
    wire [INDEX_BITS-1:0] index = cpu_addr[ADDR_WIDTH-TAG_BITS-1:OFFSET_BITS];
    wire [OFFSET_BITS-1:0] offset = cpu_addr[OFFSET_BITS-1:0];

    reg [1:0] state;
    localparam IDLE = 2'b00, COMPARE = 2'b01, WRITEBACK = 2'b10, ALLOCATE = 2'b11;

    integer i;
    reg [$clog2(WAYS)-1:0] hit_way, replace_way;
    reg hit;

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            state <= IDLE;
            cpu_ready <= 1'b0;
            mem_read <= 1'b0;
            mem_write <= 1'b0;
            for (i = 0; i < WAYS; i = i + 1) begin
                for (int j = 0; j < CACHE_SIZE/LINE_SIZE/WAYS; j = j + 1) begin
                    cache_valid[i][j] <= 1'b0;
                end
            end
        end else begin
            case (state)
                IDLE: begin
                    if (cpu_read || cpu_write) begin
                        state <= COMPARE;
                        hit <= 1'b0;
                        for (i = 0; i < WAYS; i = i + 1) begin
                            if (cache_valid[i][index] && cache_tags[i][index] == tag) begin
                                hit <= 1'b1;
                                hit_way <= i;
                            end
                        end
                    end
                end

                COMPARE: begin
                    if (hit) begin
                        if (cpu_read) begin
                            cpu_data_out <= cache_data[hit_way][index][offset[OFFSET_BITS-1:3]];
                        end else begin
                            cache_data[hit_way][index][offset[OFFSET_BITS-1:3]] <= cpu_data_in;
                        end
                        cpu_ready <= 1'b1;
                        state <= IDLE;
                    end else begin
                        replace_way <= lru_bits[index];
                        if (cache_valid[replace_way][index]) begin
                            state <= WRITEBACK;
                            mem_addr <= {cache_tags[replace_way][index], index, {OFFSET_BITS{1'b0}}};
                            mem_data_out <= cache_data[replace_way][index][0];
                            mem_write <= 1'b1;
                        end else begin
                            state <= ALLOCATE;
                            mem_addr <= {tag, index, {OFFSET_BITS{1'b0}}};
                            mem_read <= 1'b1;
                        end
                    end
                end

                WRITEBACK: begin
                    if (mem_ready) begin
                        mem_write <= 1'b0;
                        state <= ALLOCATE;
                        mem_addr <= {tag, index, {OFFSET_BITS{1'b0}}};
                        mem_read <= 1'b1;
                    end
                end

                ALLOCATE: begin
                    if (mem_ready) begin
                        mem_read <= 1'b0;
                        cache_valid[replace_way][index] <= 1'b1;
                        cache_tags[replace_way][index] <= tag;
                        cache_data[replace_way][index][0] <= mem_data_in;
                        if (cpu_write) begin
                            cache_data[replace_way][index][offset[OFFSET_BITS-1:3]] <= cpu_data_in;
                        end else begin
                            cpu_data_out <= mem_data_in;
                        end
                        cpu_ready <= 1'b1;
                        state <= IDLE;
                    end
                end
            endcase
        end
    end

    // LRU update logic
    always @(posedge clk) begin
        if (state == COMPARE && hit) begin
            for (i = 0; i < WAYS; i = i + 1) begin
                if (i == hit_way) begin
                    lru_bits[index][i] <= 1'b0;
                end else if (lru_bits[index][i] < lru_bits[index][hit_way]) begin
                    lru_bits[index][i] <= lru_bits[index][i] + 1'b1;
                end
            end
        end else if (state == ALLOCATE && mem_ready) begin
            for (i = 0; i < WAYS; i = i + 1) begin
                if (i == replace_way) begin
                    lru_bits[index][i] <= 1'b0;
                end else begin
                    lru_bits[index][i] <= lru_bits[index][i] + 1'b1;
                end
            end
        end
    end

endmodule
