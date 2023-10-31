import pandas as pd
import math

class_data = pd.read_excel("./JOE_GOSNELL1_9495.ods")
req_data = pd.read_excel("./JOE_GOSNELL2_372.ods")

with open("classes.sql", "w") as f:
    for index, row in class_data.iterrows():
        title = row.loc["Course_Title"]
        title = title.replace("\\", "")
        description = str(row.loc["Descr"])
        description = description.replace('"', "")
        req = row.loc["Rq_Group"]
        if math.isnan(req):
            req = "NULL"
        else:
            req = int(req)
        f.write(
            f"""
        CREATE course:{row.loc['Course_ID']} CONTENT {{
            title: "{title}",
            min_units: {int(row.loc['Min_Units'])},
            max_units: {int(row.loc['Max_Units'])},
            subject: "{row.loc['Subject']}",
            catalog: "{row.loc['Catalog']}",
            req_group: {req},
            description: "{description}",
        }};\n
               """
        )

with open("req.sql", "w") as f:
    for index, row in req_data.iterrows():
        course_id = row.loc["Course_ID"]
        description = str(row.loc["Descrip"])
        description = description.replace('"', "")
        if math.isnan(course_id):
            course_id = "NULL"
        else:
            course_id = int(course_id)
        f.write(
            f"""
        CREATE requisite:{row.loc['Rq_Group']} CONTENT {{
            course_id: {course_id},
            description: "{description}",
        }};\n
               """
        )
