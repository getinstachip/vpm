module dac #(parameter WIDTH = 8, parameter VREF = 1.0)(
    input  [WIDTH-1:0] digital_in, 
    output real analog_out         
);

    real resistor_value;
    real weighted_sum;
    integer i;

    always @(*) begin
        weighted_sum = 0;
        resistor_value = VREF / (2**WIDTH - 1);
        
        for (i = 0; i < WIDTH; i = i + 1) begin
            weighted_sum = weighted_sum + (digital_in[i] * (resistor_value * (2**i)));
        end
        
        analog_out = weighted_sum;
    end

endmodule
