# Run a Just recipe against a throwaway copy of the Flox environment.
#
# The recipes that check the MSRV and the dependency versions change the
# packages in the environment with `flox install` and `flox uninstall`. If they
# ran in place, they would rewrite the manifest and the lock file in the working
# tree. This script copies the tracked files to a temporary directory, renames
# the environment in the copy, and runs the recipe body there instead.
#
# Just runs it through the recipe shebang, so $1 is the recipe name and $2 is
# the temporary file that holds the recipe body.

ENV=flox-env-$1-$(date +%s)
DIR=/tmp/$ENV

rm -rf $DIR && mkdir -p $DIR
git checkout-index -a --prefix=$DIR/

cd $DIR

# The name must be unique, so that Flox treats the copy as a separate
# environment. We write a new file and move it over the old one instead of
# piping into `sponge`, because moreutils is neither in the manifest nor on the
# GitHub runners.
jq --arg name "$ENV" '.name = $name' .flox/env.json > .flox/env.json.new \
    && mv .flox/env.json.new .flox/env.json

flox activate -- sh -u <$2

# On CI we keep the temporary directory, because the runner is discarded after
# the job anyway.
if [[ -z "$CI" ]]; then
    rm -rf $DIR
fi
