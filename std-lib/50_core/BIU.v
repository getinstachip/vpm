// Bus Interface Unit
module BusInterfaceUnit (
    input wire clk,
    input wire reset,
    input wire [7:0] data_in,
    input wire [7:0] addr_in,
    input wire read,
    input wire write,
    output reg [7:0] data_out,
    output reg ready
);

    reg [7:0] memory [0:255];


    typedef enum logic [1:0] {
        IDLE,
        READ,
        WRITE
    } state_t;

    state_t current_state, next_state;

    always @(posedge clk or posedge reset) begin
        if (reset) begin
            current_state <= IDLE;
            ready <= 0;
        end else begin
            current_state <= next_state;
        end
    end


    always @(*) begin
        next_state = current_state;
        case (current_state)
            IDLE: begin
                ready = 0;
                if (read) begin
                    next_state = READ;
                end else if (write) begin
                    next_state = WRITE;
                end
            end
            READ: begin
                ready = 1;
                next_state = IDLE;
            end
            WRITE: begin
                ready = 1;
                next_state = IDLE;
            end
        endcase
    end


    always @(posedge clk) begin
        if (current_state == READ) begin
            data_out <= memory[addr_in];
        end else if (current_state == WRITE) begin
            memory[addr_in] <= data_in;
        end
    end

endmodule
