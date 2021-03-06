# How to generate all sets of P packages into N bags
Santa Claus has to optimize the space for everyone present on the sled

## Description
Santa Claus got himself a quantum computer, but the computer is still being designed by a big high tech Open Source and Open Hardware company (it's a donation to the common good for 2022). So Santa asked a bunch of developers to make small pieces of code to help him Optimize the placement of all presents in the sled bags. The algorithm that was attributed to me was a simple one: <br>

* **Generate all sets of P packages into N bags**

Others will have to make do other parts of all the christmas software, already preparing for the christmas of 2022 ... <br>
This is a nice way to start the new year, looking and developing some code for Santa Claus. <br>

## How does the algorithm works?

Well, we will do a backtracking algorithm with recurrence. In it we will transverse the stack, implicitly drawing a tree (depth first traversal) that at each node as a state for each configuration. We will be making changes to the state as we go down the tree and we will undo does changes has we come back up the tree. In the first node we will put all the presents in the first bag and then we will shift the presents between bags (from left to right) into all places, into all combinations as we travel the tree. At each node we test the constraints and collect the new presents in the bags configuration.   

```
Tree

                                    (A,B,C:_:_)
                     ____________________||____________________
                 (A,B:C:_)                                 (A,B:_:C)
           __________||__________                    __________||__________
       (A:B,C:_)             (A:C:B)              (A:B:C)            (A:_:B,C)
      _____||_____         _____||_____         _____||_____        _____||_____
(_:A,B,C:_) (_:B,C:A)  (_:A,C:B) (_:C:A,B)  (_:A,B:C) (_:B:A,C)  (_:A:B,C) (_:_:A,B,C)
```

## How many states are there?

```
               Max P
               -----
               \             p 
 Num states =   |     (b - 1)
               /
               -----
                p=0


 Where:
    "Max P" is the number of Presents or Packages.
    b is the number of Bags in the Sled.
```

## License
MIT Open Source License

## Have fun!
Best regards, <br>
Jo??o Nuno Carvalho <br>
