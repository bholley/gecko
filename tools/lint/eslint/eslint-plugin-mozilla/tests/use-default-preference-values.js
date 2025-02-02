/* Any copyright is dedicated to the Public Domain.
 * http://creativecommons.org/publicdomain/zero/1.0/ */

"use strict";

//------------------------------------------------------------------------------
// Requirements
//------------------------------------------------------------------------------

var rule = require("../lib/rules/use-default-preference-values");

//------------------------------------------------------------------------------
// Tests
//------------------------------------------------------------------------------

function invalidCode(code) {
  let message = "provide a default value instead of using a try/catch block";
  return {code: code, errors: [{message: message, type: "TryStatement"}]};
}

let types = ["Bool", "Char", "Float", "Int"];
let methods = types.map(type => "get" + type + "Pref");

exports.runTest = function(ruleTester) {
  ruleTester.run("use-ownerGlobal", rule, {
    valid: [].concat(
      methods.map(m => "blah = branch." + m + "('blah', true);"),
      methods.map(m => "blah = branch." + m + "('blah');"),
      methods.map(m => "try { canThrow();" +
                            " blah = branch." + m + "('blah'); } catch(e) {}")
    ),

    invalid: [].concat(
      methods.map(m =>
        invalidCode("try { blah = branch." + m + "('blah'); } catch(e) {}"))
    )
  });
};
