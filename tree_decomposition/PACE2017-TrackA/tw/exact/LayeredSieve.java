/*
 * Copyright (c) 2017, Hisao Tamaki
 */

package tw.exact;

import java.util.ArrayList;

public class LayeredSieve {
  int n;
  int targetWidth;
  BlockSieve sieves[];
  
  public LayeredSieve(int n, int targetWidth) {
    this.n = n;
    this.targetWidth = targetWidth;
    
    int k = 33 - Integer.numberOfLeadingZeros(targetWidth);
    sieves = new BlockSieve[k];
    for (int i = 0; i < k; i++) {
      int margin = (1 << i) - 1;
      sieves[i] = new BlockSieve(n, targetWidth, margin);
    }
  }
  
  public void put(XBitSet vertices, XBitSet neighbors) {
    int ns = neighbors.cardinality();
    int margin = targetWidth + 1 - ns;
    int i = 32 - Integer.numberOfLeadingZeros(margin);
    sieves[i].put(vertices, neighbors);
  }
  
  public void put(XBitSet vertices, int neighborSize, XBitSet value) {
    int margin = targetWidth + 1 - neighborSize;
    int i = 32 - Integer.numberOfLeadingZeros(margin);
    sieves[i].put(vertices, value);
  }
  
  public void collectSuperblocks(XBitSet component, XBitSet neighbors, 
        ArrayList<XBitSet> list) {
    for (BlockSieve sieve: sieves) {
      sieve.collectSuperblocks(component, neighbors, list);
    }
  }
  
  public int[] getSizes() {
    int sizes[] = new int[sieves.length];
    for (int i = 0; i < sieves.length; i++) {
      sizes[i] = sieves[i].size();
    }
    return sizes;
  }
}
