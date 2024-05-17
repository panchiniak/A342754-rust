This is the OEIS sequence #A342754.

I wanted to see which would run faster: node.js or rust and it came out that rust is twice as fast.

The program may be adjusted for generating a numberscope picture:

<pre>



⚫

⚫⚫

⚫⚪⚫
⚪⚫
⚫⚪⚫

⚫⚫⚫⚪⚫

⚫⚪⚪⚫
⚪⚫⚪⚫
⚫⚪⚫⚪⚪⚫

⚫⚫⚪⚫⚪⚫

⚫⚪⚫⚫⚪⚪⚫
⚪⚫⚪⚪⚫
⚫⚪⚪⚪⚪⚫

⚫⚫⚫⚪⚫⚪⚫⚪⚪⚫
⚪⚪⚫
⚫⚪⚪⚪⚪⚪⚫
⚪⚫⚪⚪⚪⚫
⚫⚪⚫⚪⚫⚪⚪⚪⚫

⚫⚫⚪⚫⚫⚪⚪⚫⚪⚪⚫

⚫⚪⚫⚪⚪⚫⚪⚪⚪⚪⚫
⚪⚫⚪⚪⚪⚪⚫
⚫⚪⚪⚪⚪⚪⚪⚪⚫
⚪⚪⚫⚪⚫
⚫⚫⚫⚪⚫⚪⚫⚪⚫⚪⚪⚪⚫

⚫⚪⚪⚪⚪⚪⚪⚪⚪⚫
⚪⚫⚪⚪⚪⚪⚪⚫
⚫⚪⚫⚫⚪⚫⚪⚫⚪⚪⚪⚪⚪⚫

⚫⚫⚪⚫⚫⚪⚪⚪⚫⚪⚪⚪⚫

⚫⚪⚫⚪⚪⚪⚫⚪⚪⚪⚪⚪⚫
⚪⚫⚪⚫⚪⚪⚫⚪⚪⚪⚫
⚫⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪⚫

⚫⚫⚫⚪⚫⚪⚫⚪⚪⚫⚪⚪⚫⚪⚪⚪⚪⚫
⚪⚪⚪⚫
⚫⚪⚫⚪⚪⚫⚪⚪⚪⚪⚪⚪⚪⚫
⚪⚫⚪⚪⚪⚪⚪⚪⚪⚫
⚫⚪⚫⚪⚪⚪⚪⚫⚪⚪⚪⚪⚪⚪⚫

⚫⚫⚪⚫⚪⚫⚪⚪⚪⚪⚫⚪⚪⚪⚪⚫
⚪⚪⚫⚪⚪⚪⚫
⚫⚪⚫⚪⚫⚫⚪⚪⚪⚫⚪⚪⚪⚪⚪⚪⚪⚫
⚪⚫⚪⚪⚪⚪⚪⚪⚪⚪⚫
⚫⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪⚫

⚫⚫⚫⚫⚫⚪⚪⚫⚪⚫⚪⚫⚪⚪⚫⚪⚪⚪⚪⚪⚫

⚫⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪⚫
⚪⚫⚪⚪⚫⚪⚫⚪⚪⚪⚪⚪⚪⚫
⚫⚪⚫⚪⚪⚫⚪⚪⚪⚪⚫⚪⚪⚪⚪⚪⚪⚪⚪⚫
⚪⚪⚫⚪⚪⚪⚪⚫
⚫⚫⚪⚫⚪⚪⚫⚪⚪⚪⚪⚪⚫⚪⚪⚪⚪⚪⚫
</pre>
Then you can use regex to look for patterns, even across multiple lines:
<pre>/^⚪⚫⚪{1}.+(\n.*){1}^⚫⚪⚫{1}/</pre>
