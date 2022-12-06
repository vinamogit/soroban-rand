cargo build --target wasm32-unknown-unknown --release

MAX=10
SAMPLES=1000
EXEC="soroban invoke --wasm target/wasm32-unknown-unknown/release/contract.wasm --id 2260611d09c76e6a7e03c1bcecf8fe02534c4ad0ddb6fd67ac96acb71b344f50  --fn roll --arg $MAX"

declare -a RESULT

for j in $(seq 0 $MAX); do
    RESULT[$j]=0
done;
unset RESULT[$MAX]

for i in $(seq 1 $SAMPLES); do
    R=$($EXEC)
    RESULT[$R]=$(( ${RESULT[$R]} + 1))
done;

function chisquare() {
    N=$1
    E=$2
    R=$3
    C=0
    for n in $(seq 0  $N); do
        C=$(echo "${C} + (${R[$n]} - $E) * (${R[$n]} - $E) / $E" | bc -l)
    done;
    echo $C
}

EI=$(( $SAMPLES / $MAX ))
N=$(( $MAX - 1 ))
CHISQUARE=$(chisquare $N $EI $RESULT)
echo ${RESULT[*]}
echo Chisquare:  $CHISQUARE

declare -a PERFECT
for k in $(seq 0 $MAX); do
    PERFECT[$k]=$EI
done;
unset PERFECT[$MAX]
PCHISQUARE=$(chisquare $N $EI $PERFECT)
echo ${PERFECT[*]}
echo Perfect:  $PCHISQUARE

SCORE=$(echo "abs($CHISQUARE - $PCHISQUARE) / $PCHISQUARE" | bc -l)
if (( $(echo "$SCORE < 0.04" |bc -l) )); then
    echo "PASS ($SCORE)"
else 
    echo "FAIL ($SCORE)"
fi
