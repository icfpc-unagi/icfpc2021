/*
 * Copyright (c) 2017, Hisao Tamaki
 */

package tw.exact;

import java.io.File;

import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.PrintStream;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashSet;

public class SafeSeparator {
  private static int MAX_MISSINGS = 100;
  private static int DEFAULT_MAX_STEPS = 1000000;
  private static final boolean CONFIRM_MINOR = true;
//  private static final boolean CONFIRM_MINOR = false;
//    private static final boolean DEBUG = true;
  private static final boolean DEBUG = false;

  Graph g;

  int maxSteps;
  int steps;
  LeftNode[] leftNodes;
  ArrayList<RightNode> rightNodeList;
  ArrayList<MissingEdge> missingEdgeList;
  XBitSet available;
  
  public SafeSeparator (Graph g) {
    this.g = g;
  }

  public boolean isSafeSeparator(XBitSet separator) {
    return isSafeSeparator(separator, DEFAULT_MAX_STEPS);
  }
  
  public boolean isSafeSeparator(XBitSet separator, int maxSteps) {
    //  System.out.println("isSafeSeparator " + separator);
    this.maxSteps = maxSteps;
    steps = 0;
    ArrayList<XBitSet> components = g.getComponents(separator);
    if (components.size() == 1) {
//      System.err.println("non separator for safety testing:" + separator);
//      throw new RuntimeException("non separator for safety testing:" + separator);
      return false;
    }
    if (countMissings(separator) > MAX_MISSINGS) {
      return false;
    }
    for (XBitSet compo: components) {
      XBitSet sep = g.neighborSet(compo);
      XBitSet rest = g.all.subtract(sep).subtract(compo);
      XBitSet[] contracts = findCliqueMinor(sep, rest);
      if (contracts == null) {
        return false;
      }
      if (CONFIRM_MINOR) {
        confirmCliqueMinor(sep, rest, contracts);
      }
    }
    return true;
  }

  private class LeftNode {
    int index;
    int vertex;
    //    ArrayList<RightNode> rightNeighborList;
    //    XBitSet rightNeighborSet;

    LeftNode(int index, int vertex) {
      this.index = index;
      this.vertex = vertex;
      //      rightNeighborList = new ArrayList<>();
    }
    
    public String toString() {
      StringBuilder sb = new StringBuilder();
      sb.append("left" + index + "(" + vertex + "):");
      sb.append(", " + g.neighborSet[vertex]);
      return sb.toString();
    }
  }

  private class RightNode {
    int index;
    XBitSet vertexSet;
    XBitSet neighborSet;
    LeftNode assignedTo;
    boolean printed;

    RightNode(int vertex) {
      vertexSet = new XBitSet(g.n);
      vertexSet.set(vertex);
      neighborSet = g.neighborSet(vertexSet);
    }

    RightNode(XBitSet vertexSet) {
      this.vertexSet = vertexSet;
      neighborSet = g.neighborSet(vertexSet);
    }
    
    boolean potentiallyCovers(MissingEdge me) {
      return 
          assignedTo == null &&
          neighborSet.get(me.left1.vertex) &&
          neighborSet.get(me.left2.vertex);
    }

    boolean finallyCovers(MissingEdge me) {
      return
          assignedTo == me.left1 &&
          neighborSet.get(me.left2.vertex) ||
          assignedTo == me.left2 &&
          neighborSet.get(me.left1.vertex);
    }

    public String toString() {
      StringBuilder sb = new StringBuilder();
      sb.append("right" + index + ":" + vertexSet);
      if (!printed) {
        sb.append(", " + neighborSet);
      }
      if (assignedTo != null) {
        sb.append("-> l" + assignedTo.index);
      }
      sb.append(", coveres {");
      for (MissingEdge me: missingEdgeList) {
        if (this.potentiallyCovers(me)) {
          sb.append("me" + me.index + " ");
        }
      }
      printed = true;
      sb.append("}");

      return sb.toString();
    }

  }

  private class MissingEdge {
    int index;
    LeftNode left1;
    LeftNode left2;
    boolean unAugmentable;

    MissingEdge(LeftNode left1, LeftNode left2) {
      this.left1 = left1;
      this.left2 = left2;
    }

    RightNode[] findCoveringPair() {
      for (RightNode rn1: rightNodeList) {
        if (rn1.neighborSet.get(left1.vertex) &&
            !rn1.neighborSet.get(left2.vertex)) {
          for (RightNode rn2: rightNodeList) {
            if (!rn2.neighborSet.get(left1.vertex) &&
                rn2.neighborSet.get(left2.vertex) &&
                connectable(rn1.vertexSet, rn2.vertexSet)) {
              return new RightNode[]{rn1, rn2}; 
            }
          }
        }
      }
      return null;
    }
    
    boolean isFinallyCovered() {
      for (RightNode rn: rightNodeList) {
        if (rn.finallyCovers(this)) {
          return true;
        }
      }
      return false;
    }
    
    public String toString() {
      StringBuilder sb = new StringBuilder();
      sb.append("missing(" + left1.index + "," + 
          left2.index + "), covered by {");
      for (RightNode rn: rightNodeList) {
        if (rn.potentiallyCovers(this)) {
          sb.append("r" + rn.index + " ");
        }
      }
      sb.append("}");
      return sb.toString();
    }
  }
  private XBitSet[] findCliqueMinor(XBitSet separator, XBitSet rest) {
    int k = separator.cardinality();
    available = (XBitSet) rest.clone();
    leftNodes = new LeftNode[k];
    {
      int i = 0;
      for (int v = separator.nextSetBit(0); v >= 0;
          v = separator.nextSetBit(v + 1)) {
        leftNodes[i] = new LeftNode(i, v);
        i++;
      }
    }
    
    missingEdgeList = new ArrayList<>();
    {
      int i = 0;
      for (int v = separator.nextSetBit(0); v >= 0;
          v = separator.nextSetBit(v + 1)) {
        int j = i + 1;
        for (int w = separator.nextSetBit(v + 1); w >= 0;
            w = separator.nextSetBit(w + 1)) {
          if (!g.neighborSet[v].get(w)) {
            missingEdgeList.add(new MissingEdge(leftNodes[i], leftNodes[j]));
          }
          j++;
        }
        i++;
      }
    }

    int m = missingEdgeList.size();
    
    XBitSet[] result = new XBitSet[k];
    for (int i = 0; i < k; i++) {
      result[i] = new XBitSet(g.n);
      result[i].set(leftNodes[i].vertex);
    }

    if (m == 0) {
      return result;
    }

//    System.out.println(m + " missings for separator size " + k + 
//        " and total components size " + rest.cardinality());
    for (int i = 0; i < m; i++) {
      missingEdgeList.get(i).index = i;
    }

    rightNodeList = new ArrayList<>();
    XBitSet ns = g.neighborSet(separator);
    ns.and(rest);

    for (int v = ns.nextSetBit(0); v >= 0; 
        v = ns.nextSetBit(v + 1)) {
      if (g.neighborSet[v].cardinality() == 1) {
        continue;
      }
      boolean useless = true;
      for (MissingEdge me: missingEdgeList) {
        if (g.neighborSet[v].get(me.left1.vertex) ||
            g.neighborSet[v].get(me.left2.vertex)) {
          useless = false;
        }
      }
      if (useless) {
        continue;
      }
      RightNode rn = new RightNode(v);
      rightNodeList.add(rn);
      available.clear(v);
    }

    while (true) {
      steps++;
      if (steps > maxSteps) {
        return null;
      }
      MissingEdge zc = zeroCovered();
      if (zc == null) {
        break;
      }
      RightNode[] coveringPair = zc.findCoveringPair();
      if (coveringPair != null) {
        mergeRightNodes(coveringPair);
      }
      else {
        return null;
      }
    }
    
    boolean moving = true;
    while (rightNodeList.size() > k/2 && moving) {
      steps++;
      if (steps > maxSteps) {
        return null;
      }
      moving = false;
      MissingEdge lc = leastCovered();
      if (lc == null) {
        break;
      }
      RightNode[] coveringPair = lc.findCoveringPair();
      if (coveringPair != null) {
        mergeRightNodes(coveringPair);
        moving = true;
      }
      else {
        lc.unAugmentable = true;
      }
    }

    ArrayList<RightNode> temp = rightNodeList;
    rightNodeList = new ArrayList<>();
    
    for (RightNode rn: temp) {
      boolean covers = false;
      for (MissingEdge me: missingEdgeList) {
        if (rn.potentiallyCovers(me)) {
          covers = true;
          break;
        }
      }
      if (covers) {
        rightNodeList.add(rn);
      }
    }
    
    int nRight = rightNodeList.size();
    for (int i = 0; i < nRight; i++) {
      rightNodeList.get(i).index = i;
    }

    if (DEBUG) {
      System.out.println(k + " lefts");
      for (LeftNode ln: leftNodes) {
        System.out.println(ln);
      }
      System.out.println(nRight + " rights");
      for (RightNode rn: rightNodeList) {
        System.out.println(rn);
      }
      System.out.println(m + " missings");
      for (MissingEdge me: missingEdgeList) {
        System.out.println(me);
      }
    }

    while (!missingEdgeList.isEmpty()) {
      if (DEBUG) {
        System.out.println(missingEdgeList.size() + " missings");
        for (RightNode rn: rightNodeList) {
          System.out.println(rn);
        }
      }
      int[] bestPair = null;
      int maxMinCover = 0;
      int maxFc = 0;

      for (LeftNode ln: leftNodes) {
        for (RightNode rn: rightNodeList) {
          if (rn.assignedTo != null ||
              !rn.neighborSet.get(ln.vertex)) {
            continue;
          }
          steps++;
          if (steps > maxSteps) {
            return null;
          }
          rn.assignedTo = ln;
          int minCover = minCover();
          int fc = 0;
          for (MissingEdge me: missingEdgeList) {
            if (me.isFinallyCovered()) {
              fc++;
            }
          }
          rn.assignedTo = null;
          if (bestPair == null || minCover > maxMinCover) {
            maxMinCover = minCover;
            bestPair = new int[] {ln.index, rn.index};
            maxFc = fc;
          }
          else if (minCover == maxMinCover && fc > maxFc) {
            bestPair = new int[] {ln.index, rn.index};
            maxFc = fc;
          }
        }
      }
      if (maxMinCover == 0) {
        return null;
      }
      
      if (DEBUG) {
        System.out.println("maxMinCover = " + maxMinCover +
            ", maxFC = " + maxFc + 
            ", bestPair = " + Arrays.toString(bestPair));
        
      }
      rightNodeList.get(bestPair[1]).assignedTo = 
          leftNodes[bestPair[0]];
      
      ArrayList<MissingEdge> temp1 = missingEdgeList;
      missingEdgeList = new ArrayList<>();
      for (MissingEdge me: temp1) {
        if (!me.isFinallyCovered()) {
          missingEdgeList.add(me);
        }
      }
    }

    if (DEBUG) {
      System.out.println("assignment success");
      for (RightNode rn: rightNodeList) {
        System.out.println(rn);
      }
    }
    
    for (RightNode rn: rightNodeList) {
      if (rn.assignedTo != null) {
        int i = rn.assignedTo.index;
        result[i].or(rn.vertexSet);
      }
    }
    return result;
  }

  void confirmCliqueMinor(XBitSet separator, XBitSet rest, XBitSet[] contracts) {
    {
      int i = 0;
      for (int v = separator.nextSetBit(0); v >= 0;
          v = separator.nextSetBit(v + 1)) {
        if (!contracts[i].get(v)) {
          throw new RuntimeException("Not a clique minor: vertex " + v + 
              " is not contained in the contracted " + contracts[i]);
        }
        i++;
      }
    }
    for (int i = 0; i < contracts.length; i++) {
      for (int j = i + 1; j < contracts.length; j++) {
        if (contracts[i].intersects(contracts[j])) {
          throw new RuntimeException("Not a clique minor: contracts " + 
              contracts[i] + " and " + contracts[j] + " intersect with each other");
        }
        if (!g.neighborSet(contracts[i]).intersects(contracts[j])) {
          throw new RuntimeException("Not a clique minor: contracts " + 
              contracts[i] + " and " + contracts[j] + " are not adjacent to each other");
        }
      }
    }
    
    for (int i = 0; i < contracts.length; i++) {
      if (!g.isConnected(contracts[i])) {
        throw new RuntimeException("Not a clique minor: contracted " + 
            contracts[i] + " is not connected");
      }
    }
  }
  
  int minCover() {
    int minCover = g.n;
    for (MissingEdge me: missingEdgeList) {
      if (me.isFinallyCovered()) {
        continue;
      }
      int nCover = 0;
      for (RightNode rn: rightNodeList) {
        if (rn.potentiallyCovers(me)) {
          nCover++;
        }
      }
      if (nCover < minCover) {
        minCover = nCover;
      }
    }
    return minCover;    
  }

  MissingEdge leastCovered() {
    int minCover = 0;
    MissingEdge result = null;
    for (MissingEdge me: missingEdgeList) {
      if (me.unAugmentable) {
        continue;
      }
      int nCover = 0;
      for (RightNode rn: rightNodeList) {
        if (rn.potentiallyCovers(me)) {
          nCover++;
        }
      }
      if (result == null || nCover < minCover) {
        minCover = nCover;
        result =  me;
      }
    }
    return result;    
  }

  MissingEdge zeroCovered() {
    for (MissingEdge me: missingEdgeList) {
      int nCover = 0;
      for (RightNode rn: rightNodeList) {
        if (rn.potentiallyCovers(me)) {
          nCover++;
        }
      }
      if (nCover == 0) {
        return me;
      }
    }
    return null;    
  }

  boolean connectable(XBitSet vs1, XBitSet vs2) {
    XBitSet vs = (XBitSet) vs1.clone();
    while (true) {
      XBitSet ns = g.neighborSet(vs);
      if (ns.intersects(vs2)) {
        return true;
      }
      ns.and(available);
      if (ns.isEmpty()) {
        return false;
      }
      vs.or(ns);
    }
  }

  void mergeRightNodes(RightNode[] coveringPair) {
    RightNode rn1 = coveringPair[0];
    RightNode rn2 = coveringPair[1];

    XBitSet connected = connect(rn1.vertexSet, rn2.vertexSet);
    RightNode rn = new RightNode(connected);
    rightNodeList.remove(rn1);
    rightNodeList.remove(rn2);
    rightNodeList.add(rn);
  }

  XBitSet connect(XBitSet vs1, XBitSet vs2) {
    ArrayList<XBitSet> layerList = new ArrayList<>();
    
    XBitSet vs = (XBitSet) vs1.clone();
    while (true) {
      XBitSet ns = g.neighborSet(vs);
      if (ns.intersects(vs2)) {
        break;
      }
      ns.and(available);
      layerList.add(ns);
      vs.or(ns);
    }
    
    XBitSet result = vs1.unionWith(vs2); 
    
    XBitSet back = g.neighborSet(vs2);
    for (int i = layerList.size() - 1; i >= 0; i--) {
      XBitSet ns = layerList.get(i);
      ns.and(back);
      int v = ns.nextSetBit(0);
      result.set(v);
      available.clear(v);
      back = g.neighborSet[v];
    }    
    return result;
  }
  
  int countMissings(XBitSet s) {
    int count = 0;
    for (int v = s.nextSetBit(0); v >= 0;
        v = s.nextSetBit(v + 1)) {
      count += s.subtract(g.neighborSet[v]).cardinality() - 1;
    }
    return count / 2;
  }

}
