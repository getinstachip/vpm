module dff(d,clk,q);
    input d,clk;
    output q;
    reg q=0;
    always @ (posedge clk)
        begin
            q<=d;
        end
endmodule