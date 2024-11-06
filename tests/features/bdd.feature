Feature: Bdd Feature

  Scenario: If we provide a true input to a not bdd, we receive false
    Given a bdd with definition
      """
      vars 1
      nodes 3
      0 2 1 0
      1 -1 -1 1
      2 -1 -1 0
      """
    When 01 is assigned as hex
    Then the evaluation should be false

  Scenario: If we provide an false input to an identity bdd, we receive false
    Given a bdd with definition
      """
      vars 1
      nodes 3
      0 2 1 0
      1 -1 -1 0
      2 -1 -1 1
      """
    When 00 is assigned as hex
    Then the evaluation should be false

  Scenario: Truth table for not bdd
    Given a bdd with definition
      """
      vars 1
      nodes 3
      0 2 1 0
      1 -1 -1 1
      2 -1 -1 0
      """
    Then the truth table should equal
      """
      0 = true
      1 = false
      """

  Scenario: Truth table for an identity bdd
    Given a bdd with definition
      """
      vars 1
      nodes 3
      0 2 1 0
      1 -1 -1 0
      2 -1 -1 1
      """
    Then the truth table should equal
      """
      0 = false
      1 = true
      """
