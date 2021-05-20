# Grid_Value
A tool to find the 16-bit value of a 4x4 grid.

I've been using it to find the values of tetris pieces so I don't have to store them as a grid of booleans. Honestly can't imagine any other use for this. 

Features stunning graphics:

![image](https://user-images.githubusercontent.com/7970508/119032630-67968b00-b97a-11eb-83d7-b19bdee853e0.png)

Update: I've gone ahead and calculated the values so no one ever has to use this. The values follow the official [Super Rotation System](https://tetris.fandom.com/wiki/SRS?file=SRS-pieces.png). All pieces start in the upper-left-hand corner, with the exception of the O-piece. For whatever reason, SRS demands that this block is shifted to the right by one unit. "0" represents the pieces' starting rotation states, and each value after is a 90 degree roation. 

* I piece:
  * 0: 3840
  * 1: 8738
  * 2: 240
  * 3: 17476
* J piece:
	* 0: 36352
	* 1: 25664
	* 2: 3616
	* 3: 17600
* L piece:
	* 0: 11776
	* 1: 17504
	* 2: 3712
	* 3: 50240
* O piece: 
	* 0: 26112
	* 1: 26112
	* 2: 26112
	* 3: 26112
* S piece:
	* 0: 27648
	* 1: 17952
	* 2: 1728
	* 3: 35904
* T piece:
	* 0: 19968
	* 1: 17984
	* 2: 3648
	* 3: 19520
* Z piece:
	* 0: 50688
	* 1: 9792
	* 2: 3168
	* 3: 19584
