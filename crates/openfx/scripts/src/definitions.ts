export interface CodegenConfig {
  property_sets: {
    renaming: Record<string, string>;
  };
  properties: {
    names: {
      simple_names_regex: string;
      simple_names_special_cases: Record<string, string>;
    };
  };
  actions: {
    pseudo: {
      members: string[];
    };
    core: {
      members_regex: string;
      members: Record<
        string,
        { handle: ActionMemberHandle; in: boolean; out: boolean }
      >;
    };
    image_effect: {
      handle_prefix: string;
      members_from_core: string[];
      members_regex: string;
      members: Record<
        string,
        { handle: ActionMemberHandle; in: boolean; out: boolean }
      >;
    };
    interact: {
      handle_prefix: string;
      members_from_core: string[];
      members_regex: string;
      members: Record<
        string,
        { handle: ActionMemberHandle; in: boolean; out: boolean }
      >;
    };
  };
}

type ActionMemberHandle = "Instance" | "Descriptor" | false;
