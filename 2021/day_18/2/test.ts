const fs = require("fs");

const lines: string[] = fs.readFileSync("input", "utf-8").split("\n").filter(l => l);

type TreeNode =
    | { left: TreeNode; right: TreeNode; parent?: TreeNode }
    | { value: number; parent?: TreeNode }
    ;

function parseLine(line: string, idx: number): [TreeNode, number] {
    if (idx === 0)
        console.log("parse", line);
    if (line[idx] === '[') {
        idx += 1;
        const [left, i1] = parseLine(line, idx);
        idx = i1 + 1; // ,
        const [right, i2] = parseLine(line, idx);
        idx = i2 + 1; // ]
        const node = { left, right };
        left.parent = node;
        right.parent = node;
        return [node, idx];
    }
    else if (/^[0-9]/.test(line[idx])) {
        let value = parseInt(line[idx++]);
        while (/^[0-9]/.test(line[idx])) {
            value = value * 10 + parseInt(line[idx++]);
        }
        return [{ value }, idx];
    }
    throw new Error(`unexpected ${line[idx]} at ${idx}`);
}

function add(left: TreeNode, right: TreeNode): TreeNode {
    const node = { left, right };
    left.parent = node;
    right.parent = node;
    return node;
}

function explode(tree: TreeNode, depth: number): boolean {
    if ("value" in tree) {
        return false;
    }
    const { left, right, parent } = tree;
    if (depth < 4)
        return explode(left, depth + 1) || explode(right, depth + 1);
    if (depth > 4)
        throw new Error(`explode(${tree}, ${depth}) shouldn't happen`);

    if ("left" in left || "left" in right || !parent || "value" in parent || !parent.parent)
        throw new Error(`explode(${tree}, ${depth}) shouldn't happen`);

    if (parent.left === tree) {
        parent.left = { value: 0, parent };
        tree = parent.left;
    }
    else if (parent.right === tree) {
        parent.right = { value: 0, parent };
        tree = parent.right;
    }

    const leftSibling = findLeftSiblingValue(tree);
    if (leftSibling && "value" in leftSibling)
        leftSibling.value += left.value;
    const rightSibling = findRightSiblingValue(tree);
    if (rightSibling && "value" in rightSibling)
        rightSibling.value += right.value;

    return true;
}

function findLeftSiblingValue(tree: TreeNode): TreeNode | undefined {
    if (!tree.parent)
        return undefined;
    let parent = tree.parent as TreeNode | undefined;
    while (parent && "left" in parent && parent.left === tree) {
        tree = parent;
        parent = tree.parent;
    }
    if (parent && "right" in parent && tree !== parent.left) {
        tree = parent.left;
        while ("right" in tree)
            tree = tree.right;
        return tree;
    }
    return undefined;
}

function findRightSiblingValue(tree: TreeNode): TreeNode | undefined {
    if (!tree.parent)
        return undefined;
    let parent = tree.parent as TreeNode | undefined;
    while (parent && "right" in parent && parent.right === tree) {
        tree = parent;
        parent = tree.parent;
    }
    if (parent && "left" in parent && tree !== parent.right) {
        tree = parent.right;
        while ("left" in tree)
            tree = tree.left;
        return tree;
    }
    return undefined;
}

function split(tree: TreeNode): boolean {
    if ("value" in tree) {
        if (tree.value >= 10) {
            const ntree: any = tree;
            ntree.left = { parent: tree, value: Math.floor(tree.value / 2) };
            ntree.right = { parent: tree, value: Math.ceil(tree.value / 2) };
            delete tree.value;
            return true;
        }
        return false;
    }
    return split(tree.left) || split(tree.right);
}

function str(tree: TreeNode, indent: number = 0) {
    if ("value" in tree) {
        return String(tree.value);
    }
    return "[" + str(tree.left) + "," + str(tree.right) + "]";
}

function magnitude(tree: TreeNode) {
    if ("value" in tree)
        return tree.value;
    return magnitude(tree.left) * 3 + magnitude(tree.right) * 2;
}

function copy(tree: TreeNode): TreeNode {
    const clone = { ...tree };
    if ("right" in clone) {
        clone.left = copy(clone.left);
        clone.left.parent = clone;
        clone.right = copy(clone.right);
        clone.right.parent = clone;
    }
    return clone;
}


let trees = lines.map(l => parseLine(l, 0)[0]);

let maxMag = 0;
for (const left of trees) {
    for (const right of trees) {
        if (left === right)
            continue;
        const tree = add(copy(left), copy(right));
        while (explode(tree, 0) || split(tree));
        const mag = magnitude(tree);
        console.log(str(left), " + ", str(right), " = ", mag);
        maxMag = Math.max(maxMag, mag);
    }
}
console.log("Max = ", maxMag);
