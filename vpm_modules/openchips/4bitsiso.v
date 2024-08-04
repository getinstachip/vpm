module siso(d, clk, q);
    input d,clk;
    output q;
    wire q1,q2,q3;
    dff a(d, clk, q1);
    dff b(q1, clk, q2);
    dff c(q2, clk, q3);
    dff d1(q3, clk, q);
endmodule